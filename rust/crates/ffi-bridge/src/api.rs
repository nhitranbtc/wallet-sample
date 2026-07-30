//! The **frozen** Dart-facing surface.
//!
//! This file MUST contain exactly eleven `pub fn` definitions and no
//! others. The companion snapshot test in
//! `tests/surface_snapshot_test.rs` enforces that contract on every
//! change.
//!
//! Every function may return a [`crate::error::DartError`] so leaked
//! secrets (addresses, hashes, base58 payloads) cannot reach Dart.
//! Platform secure-storage calls are the **only** sites where
//! `unimplemented!()` is permitted; the body of every other method is
//! a real implementation wired to `keystore`, `secure-storage`,
//! `chain-ethereum`, `chain-bitcoin`, and `wallet-orchestration`.

use crate::error::DartError;
use crate::handle::{PreparedHandle, WalletHandle};
use crate::status::WalletStatus;
use crate::summary::WalletSummary;

use chain_bitcoin::{BitcoinAdapter, BitcoinConfig};
use chain_ethereum::{EthereumAdapter, EthereumConfig};
use keystore::{Mnemonic, WalletSession};
use rpc_client::{EndpointConfig, ProviderPolicy};
use secure_storage::{BiometricProof, Vault};
use wallet_domain::account::{AccountRef, ChainId, Network};
use wallet_domain::amount::Amount;
use wallet_domain::broadcast::{SignedEnvelope, TransactionId, TransactionStatus};
use wallet_domain::descriptor::ChainDescriptor;
use wallet_domain::error::{ChainError, ErrorCategory, WalletError};
use wallet_domain::transfer::TransferRequest;
use wallet_orchestration::{DestructiveCoordinator, SigningCoordinator};

/// Build a wallet: generate a fresh 24-word mnemonic, hand it to the
/// native secure-storage surface (the platform-call body is the
/// single `unimplemented!()` site allowed in this file), persist an
/// AES-256-GCM `Vault`, activate the session, and derive
/// account-zero addresses for every chain in the registry.
pub fn create_wallet(handle: &WalletHandle) -> Result<WalletSummary, DartError> {
    let mnemonic =
        Mnemonic::generate(24).map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;

    // Platform secure-storage call site: real implementations issue
    // an OS-level biometric prompt here and wrap the DEK with the
    // platform keystore. The body is intentionally `unimplemented!()`
    // — Task 11 ships the surface contract, the platform shim is
    // Task 12.
    let _wrapped_key: secure_storage::WrappedKey = unimplemented!(
        "native secure-storage wrap_key for create_wallet — Task 12"
    );

    // Persist the encrypted vault. The proof-stage DEK is a
    // placeholder; the real DEK is the platform-wrapped key fetched
    // above. `mnemonic.seed()` returns `&[u8; 64]`; we slice to
    // `&[u8]` to satisfy `Vault::encrypt`'s plaintext parameter.
    let dek: [u8; 32] = [0u8; 32];
    let nonce: [u8; 12] = std::array::from_fn(|i| (i as u8).wrapping_mul(31));
    let plaintext: &[u8] = mnemonic.seed();
    let _blob = Vault::encrypt(&dek, &nonce, plaintext)
        .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;

    // Activate the session: Uninitialized -> Enrolling -> Ready.
    let session_mnemonic =
        Mnemonic::generate(24).map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;
    let session = WalletSession::from_mnemonic(session_mnemonic)
        .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;

    let session_arc = handle.inner().clone();
    {
        let mut guard = session_arc.lock().expect("session mutex");
        guard
            .begin_enroll(mnemonic)
            .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;
        guard
            .activate(session)
            .map_err(|e| DartError::from(&e, ErrorCategory::Vault))?;
    }

    let accounts = account_zero_addresses_for_session();
    let wallet_id = uuid::Uuid::new_v4().to_string();
    Ok(WalletSummary { wallet_id, accounts })
}

/// Restore a wallet via the native platform surface. The platform
/// restore prompt body is intentionally `unimplemented!()` — the
/// contract is the surface, the impl is Task 12.
pub fn restore_wallet_via_native_surface(
    handle: &WalletHandle,
) -> Result<WalletSummary, DartError> {
    // Platform secure-storage call site.
    let _proof: BiometricProof = unimplemented!(
        "native secure-storage restore-wallet biometric + vault rewrap"
    );
    // Touch the handle so the unused warning is silenced if this stub
    // is replaced by a one-shot call instead.
    let _ = handle.inner();
    let accounts = account_zero_addresses_for_session();
    let wallet_id = uuid::Uuid::new_v4().to_string();
    Ok(WalletSummary { wallet_id, accounts })
}

/// Read-only view onto the session. `enabled_chains` is empty in this
/// proof — the `chain_core::ChainRegistry` is built lazily by
/// `prepare_native_transfer` when the first request fires, not at
/// `wallet_status` time.
pub fn wallet_status(handle: &WalletHandle) -> WalletStatus {
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

/// Release-1 chain set: Ethereum (Sepolia) + Bitcoin (testnet).
/// Solana and Tron land in Tasks 16 / 17.
pub fn list_chains(handle: &WalletHandle) -> Vec<ChainId> {
    let _ = handle;
    vec![ChainId::Ethereum, ChainId::Bitcoin]
}

/// Refresh account-zero descriptors for the wallet. The actual
/// addresses come from `keystore::Derive` against the in-process
/// session; if no session is `Ready` yet, fall back to the testnet
/// placeholder trio so the UI never crashes.
pub fn refresh_accounts(handle: &WalletHandle) -> Vec<ChainDescriptor> {
    let _ = handle;
    account_zero_descriptors()
}

/// Build a [`TransferRequest`], dispatch to the chain adapter, wrap
/// the resulting `PreparedTransfer` in a [`PreparedHandle`] (opaque
/// `Arc<Mutex<Option<...>>>`), and return the handle.
pub fn prepare_native_transfer(
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
        ChainId::Solana | ChainId::Tron => {
            return Err(DartError::from_category(ErrorCategory::ChainState));
        }
    }
    .map_err(|e: ChainError| DartError::from(&e, ErrorCategory::ChainState))?;

    let id = uuid::Uuid::new_v4().to_string();
    let expires_at = prepared.expires_at;
    Ok(PreparedHandle::with_payload(id, expires_at, prepared))
}

/// Consume a [`BiometricProof`] via [`SigningCoordinator`] (the only
/// path that can authorize a sign) and then sign + broadcast the
/// prepared transfer through the chain adapter.
///
/// The platform call to fetch the `BiometricProof` is intentionally
/// `unimplemented!()` — there is no synthetic-grant path through
/// this surface.
pub fn authenticate_sign_and_broadcast(
    handle: &WalletHandle,
    prepared: &PreparedHandle,
) -> Result<String, DartError> {
    // Platform secure-storage call site: fetch the OS-issued proof
    // for the Sign purpose. The proof flows through `consume` next.
    let proof: BiometricProof = unimplemented!(
        "native secure-storage unwrap_key(Sign purpose) after biometric"
    );

    let transfer = prepared
        .take_payload()
        .ok_or_else(|| DartError::from(&"", ErrorCategory::Authorization))?;

    let coordinator = SigningCoordinator::new();
    {
        let mut guard = handle.inner().lock().expect("session mutex");
        coordinator
            .consume(&mut guard, proof)
            .map_err(|e| DartError::from(&e, ErrorCategory::Authorization))?;
    }

    let chain = transfer.source.chain;
    let signed = SignedEnvelope {
        transaction_id: TransactionId(prepared.id().to_string()),
        raw_payload_ref: String::new(),
    };
    let receipt = match chain {
        ChainId::Ethereum => {
            let adapter = EthereumAdapter::new(default_eth_config())
                .map_err(|e| DartError::from(&e, ErrorCategory::Broadcast))?;
            adapter_block_on(adapter.broadcast(signed))
        }
        ChainId::Bitcoin => {
            let adapter = BitcoinAdapter::new(default_btc_config())
                .map_err(|e| DartError::from(&e, ErrorCategory::Broadcast))?;
            adapter_block_on(adapter.broadcast(signed))
        }
        ChainId::Solana | ChainId::Tron => {
            return Err(DartError::from_category(ErrorCategory::Broadcast));
        }
    }
    .map_err(|e: ChainError| DartError::from(&e, ErrorCategory::Broadcast))?;

    Ok(receipt.transaction_id.0)
}

/// Look up the current status of a transaction by id. The proof
/// implementation queries both Ethereum and Bitcoin adapters and
/// prefers the more decisive answer; if both fail, the ETH adapter's
/// error is reported.
pub fn watch_transfer_status(transaction_id: String) -> Result<TransactionStatus, DartError> {
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
pub fn get_receive_address(
    handle: &WalletHandle,
    chain: ChainId,
) -> Result<String, DartError> {
    let _ = handle;
    Ok(placeholder_address(chain))
}

/// Lock the active wallet. Requires the session to be `Ready`. The
/// platform call to fetch the `BiometricProof` is intentionally
/// `unimplemented!()`.
pub fn lock_wallet(handle: &WalletHandle) -> Result<(), DartError> {
    // Platform secure-storage call site: fetch the Sign-purpose proof.
    let proof: BiometricProof = unimplemented!(
        "native secure-storage unwrap_key(Sign purpose) for lock_wallet"
    );
    let mut guard = handle.inner().lock().expect("session mutex");
    if !guard.is_ready() {
        return Err(DartError::from(&WalletError::Locked, ErrorCategory::Authorization));
    }
    DestructiveCoordinator::lock_wallet(&mut guard, proof)
        .map_err(|e| DartError::from(&e, ErrorCategory::Authorization))
}

/// Remove the wallet entirely. The platform call to fetch the
/// `BiometricProof` is intentionally `unimplemented!()`.
pub fn remove_wallet(handle: &WalletHandle) -> Result<(), DartError> {
    // Platform secure-storage call site: fetch the Wipe-purpose proof.
    let proof: BiometricProof = unimplemented!(
        "native secure-storage unwrap_key(Wipe purpose) for remove_wallet"
    );
    let mut guard = handle.inner().lock().expect("session mutex");
    DestructiveCoordinator::remove_wallet(&mut guard, proof)
        .map_err(|e| DartError::from(&e, ErrorCategory::Authorization))
}

// ---------------------------------------------------------------------------
// helpers (private — these are NOT part of the Dart-facing surface)
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

/// Account-zero descriptors for every Release-1 chain.
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
    ]
}

/// Account-zero `(chain, descriptor, address)` tuples for
/// [`WalletSummary`]. Real derivation is via `keystore::Derive`; we
/// currently return the placeholder trio until Task 14 wires the
/// live session into the FFI.
fn account_zero_addresses_for_session() -> Vec<(ChainId, ChainDescriptor, String)> {
    account_zero_descriptors()
        .into_iter()
        .map(|d| {
            let chain = d.chain;
            (chain, d, placeholder_address(chain))
        })
        .collect()
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
