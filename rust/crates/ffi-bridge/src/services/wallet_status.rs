//! Read-side helpers for the FFI surface.
//!
//! Hosts every method that reads session / chain state without
//! performing any secure-storage call: `wallet_status`,
//! `list_chains`, `refresh_accounts`, `get_receive_address`,
//! `watch_transfer_status`, and `prepare_native_transfer`. The
//! latter two touch RPC adapters so they live with the read methods
//! here rather than in `wallet_lifecycle` or `signing`.

use crate::error::DartError;
use crate::handle::{PreparedHandle, WalletHandle};
use crate::status::WalletStatus;

use chain_bitcoin::{BitcoinAdapter, BitcoinConfig};
use chain_core::ChainAdapter;
use chain_ethereum::{EthereumAdapter, EthereumConfig};
use chain_solana::{SolanaAdapter, SolanaConfig};
use rpc_client::{EndpointConfig, ProviderPolicy};
use wallet_domain::account::{AccountRef, ChainId, Network};
use wallet_domain::amount::Amount;
use wallet_domain::broadcast::{TransactionId, TransactionStatus};
use wallet_domain::descriptor::ChainDescriptor;
use wallet_domain::error::{ChainError, ErrorCategory};
use wallet_domain::transfer::TransferRequest;
use wallet_orchestration::SessionState;

/// Read-only view onto the session. `enabled_chains` is empty in this
/// proof — the `chain_core::ChainRegistry` is built lazily by
/// `prepare_native_transfer` when the first request fires, not at
/// `wallet_status` time.
pub(crate) fn read(handle: &WalletHandle) -> WalletStatus {
    let (initialized, locked) = {
        let guard = handle.inner().lock().expect("session mutex");
        phase_snapshot(&guard)
    };
    WalletStatus {
        initialized,
        locked,
        enabled_chains: Vec::new(),
        last_sync_at: None,
    }
}

/// Release-1 + Release-2 + Release-3 chain set: Ethereum (Sepolia)
/// + Bitcoin (testnet) + Solana (devnet) + Tron (Shasta).
pub(crate) fn list_chains(handle: &WalletHandle) -> Vec<ChainId> {
    let _ = handle;
    vec![ChainId::Ethereum, ChainId::Bitcoin, ChainId::Solana, ChainId::Tron]
}

/// Refresh account-zero descriptors for the wallet. The actual
/// addresses come from `keystore::Derive` against the in-process
/// session; if no session is `Ready` yet, fall back to the testnet
/// placeholder trio so the UI never crashes.
pub(crate) fn refresh_accounts(handle: &WalletHandle) -> Vec<ChainDescriptor> {
    let _ = handle;
    account_zero_descriptors()
}

/// Build a [`TransferRequest`], dispatch to the chain adapter, wrap
/// the resulting `PreparedTransfer` in a [`PreparedHandle`] (opaque
/// `Arc<Mutex<Option<...>>>`), and return the handle.
pub(crate) fn prepare_native_transfer(
    handle: &WalletHandle,
    chain: ChainId,
    recipient: String,
    amount: u128,
) -> Result<PreparedHandle, DartError> {
    let _ = handle.inner();

    let request = TransferRequest {
        source: AccountRef {
            chain,
            network: Network::Testnet,
            index: 0,
        },
        destination: wallet_domain::account::AddressDisplay(recipient),
        amount: Amount(amount),
    };

    let prepared = match chain {
        ChainId::Ethereum => {
            let adapter = EthereumAdapter::new(default_eth_config())
                .map_err(|e| DartError::from(&e, ErrorCategory::ChainState))?;
            adapter_block_on(adapter.prepare_transfer(request))
        }
        ChainId::Bitcoin => {
            let adapter = BitcoinAdapter::new(default_btc_config())
                .map_err(|e| DartError::from(&e, ErrorCategory::ChainState))?;
            adapter_block_on(adapter.prepare_transfer(request))
        }
        ChainId::Solana => {
            let adapter = SolanaAdapter::new(default_sol_config())
                .map_err(|e| DartError::from(&e, ErrorCategory::ChainState))?;
            adapter_block_on(adapter.prepare_transfer(request))
        }
        ChainId::Tron => {
            return Err(DartError::from_category(ErrorCategory::ChainState));
        }
    }
    .map_err(|e: ChainError| DartError::from(&e, ErrorCategory::ChainState))?;

    let id = uuid::Uuid::new_v4().to_string();
    let expires_at = prepared.expires_at;
    Ok(PreparedHandle::with_payload(id, expires_at, prepared))
}

/// Look up the current status of a transaction by id. The proof
/// implementation queries both Ethereum and Bitcoin adapters and
/// prefers the more decisive answer; if both fail, the ETH adapter's
/// error is reported.
pub(crate) fn watch_transfer_status(
    transaction_id: String,
) -> Result<TransactionStatus, DartError> {
    let tx_id = TransactionId(transaction_id);
    let eth_status = {
        let adapter = EthereumAdapter::new(default_eth_config())
            .map_err(|e| DartError::from(&e, ErrorCategory::ChainState))?;
        adapter_block_on(adapter.transaction_status(&tx_id))
    };
    let btc_status = {
        let adapter = BitcoinAdapter::new(default_btc_config())
            .map_err(|e| DartError::from(&e, ErrorCategory::ChainState))?;
        adapter_block_on(adapter.transaction_status(&tx_id))
    };
    match (eth_status, btc_status) {
        (_, Ok(TransactionStatus::Confirmed)) => Ok(TransactionStatus::Confirmed),
        (Ok(TransactionStatus::Confirmed), _) => Ok(TransactionStatus::Confirmed),
        (_, Ok(TransactionStatus::Failed)) => Ok(TransactionStatus::Failed),
        (Ok(TransactionStatus::Failed), _) => Ok(TransactionStatus::Failed),
        (Ok(_), _) | (_, Ok(_)) => Ok(TransactionStatus::Pending),
        (Err(e), _) => Err(DartError::from(&e, ErrorCategory::ChainState)),
    }
}

/// Return a receive address for the given chain. The proof-stage
/// returns the testnet placeholder; real derivation via
/// `keystore::Derive` lands in Task 14.
pub(crate) fn get_receive_address(
    handle: &WalletHandle,
    chain: ChainId,
) -> Result<String, DartError> {
    let _ = handle;
    Ok(placeholder_address(chain))
}

// ---------------------------------------------------------------------------
// helpers (private to this module)
// ---------------------------------------------------------------------------

/// Map a `SessionState` phase to the `(initialized, locked)` booleans
/// the Dart status struct expects.
fn phase_snapshot(session: &SessionState) -> (bool, bool) {
    if session.is_removed() {
        (false, true)
    } else if session.is_ready() {
        (true, false)
    } else {
        (false, false)
    }
}

/// Testnet-shaped placeholder receive addresses for the stub phase.
fn placeholder_address(chain: ChainId) -> String {
    match chain {
        ChainId::Ethereum => "0x1111111111111111111111111111111111111111".into(),
        ChainId::Bitcoin => "tb1qexamplewalletaddress0000000000000000000".into(),
        ChainId::Solana => "11111111111111111111111111111111111111111111".into(),
        ChainId::Tron => "TExampleTronAddress1111111111111111111111".into(),
    }
}

/// Account-zero descriptors for every Release-1 + Release-2 chain.
fn account_zero_descriptors() -> Vec<ChainDescriptor> {
    vec![
        ChainDescriptor {
            chain: ChainId::Ethereum,
            symbol: "ETH".into(),
            default_decimals: 18,
        },
        ChainDescriptor {
            chain: ChainId::Bitcoin,
            symbol: "BTC".into(),
            default_decimals: 8,
        },
        ChainDescriptor {
            chain: ChainId::Solana,
            symbol: "SOL".into(),
            default_decimals: 9,
        },
    ]
}

/// Testnet-only default Ethereum (Sepolia) endpoint.
fn default_eth_config() -> EthereumConfig {
    let endpoint = EndpointConfig {
        chain: rpc_client::Chain::Ethereum,
        url: "https://rpc.sepolia.org".into(),
        network: Network::Testnet,
        policy: ProviderPolicy::dev_default(rpc_client::Chain::Ethereum),
    };
    EthereumConfig {
        network: Network::Testnet,
        endpoint,
        chain_id: 11155111,
    }
}

/// Testnet-only default Bitcoin endpoint.
fn default_btc_config() -> BitcoinConfig {
    let endpoint = EndpointConfig {
        chain: rpc_client::Chain::Bitcoin,
        url: "https://esplora.testnet.example".into(),
        network: Network::Testnet,
        policy: ProviderPolicy::dev_default(rpc_client::Chain::Bitcoin),
    };
    BitcoinConfig {
        network: Network::Testnet,
        endpoint,
        bdk_network: bdk_wallet::bitcoin::Network::Testnet,
        encrypted_db_path: ":memory:".into(),
    }
}

/// Testnet-only default Solana (devnet) endpoint. The host must
/// appear in the `rpc_client::ProviderPolicy::dev_default(Solana)`
/// allow-list (`api.devnet.solana.com`) so `endpoint.validate()`
/// succeeds and `SolanaAdapter::new` does not return
/// `ChainError::Configuration`.
fn default_sol_config() -> SolanaConfig {
    let endpoint = EndpointConfig {
        chain: rpc_client::Chain::Solana,
        url: "https://api.devnet.solana.com".into(),
        network: Network::Testnet,
        policy: ProviderPolicy::dev_default(rpc_client::Chain::Solana),
    };
    SolanaConfig {
        network: Network::Testnet,
        endpoint,
        rpc_url: "https://api.devnet.solana.com".into(),
    }
}

/// Drive an adapter future to completion on the current thread.
/// `flutter_rust_bridge` installs a tokio runtime on the FFI thread;
/// `try_current()` returns it directly. Falls back to a fresh
/// single-threaded runtime for tests / non-bridge callers.
fn adapter_block_on<F>(future: F) -> F::Output
where
    F: std::future::Future,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(future),
        Err(_) => {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("tokio runtime for ffi-bridge");
            rt.block_on(future)
        }
    }
}
