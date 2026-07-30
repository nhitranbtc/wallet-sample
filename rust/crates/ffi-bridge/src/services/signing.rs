//! Signing helpers: `authenticate_sign_and_broadcast`.
//!
//! Consumes a [`secure_storage::BiometricProof`] via
//! [`wallet_orchestration::SigningCoordinator`] (the only path that
//! can authorize a sign) and then signs + broadcasts the prepared
//! transfer through the chain adapter. The platform call to fetch
//! the `BiometricProof` is intentionally `unimplemented!()` — there
//! is no synthetic-grant path through this surface.

use crate::error::DartError;
use crate::handle::{PreparedHandle, WalletHandle};

use chain_bitcoin::{BitcoinAdapter, BitcoinConfig};
use chain_ethereum::{EthereumAdapter, EthereumConfig};
use rpc_client::{EndpointConfig, ProviderPolicy};
use secure_storage::BiometricProof;
use wallet_domain::account::ChainId;
use wallet_domain::broadcast::{SignedEnvelope, TransactionId};
use wallet_domain::error::{ChainError, ErrorCategory};
use wallet_orchestration::SigningCoordinator;

/// Consume a [`BiometricProof`] via [`SigningCoordinator`] and then
/// sign + broadcast the prepared transfer through the chain adapter.
pub(crate) fn authenticate_sign_and_broadcast(
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

/// Testnet-only default Ethereum (Sepolia) endpoint.
fn default_eth_config() -> chain_ethereum::EthereumConfig {
    let endpoint = EndpointConfig {
        chain: rpc_client::Chain::Ethereum,
        url: "https://rpc.sepolia.org".into(),
        network: wallet_domain::account::Network::Testnet,
        policy: ProviderPolicy::dev_default(rpc_client::Chain::Ethereum),
    };
    chain_ethereum::EthereumConfig {
        network: wallet_domain::account::Network::Testnet,
        endpoint,
        chain_id: 11155111,
    }
}

/// Testnet-only default Bitcoin endpoint.
fn default_btc_config() -> BitcoinConfig {
    let endpoint = EndpointConfig {
        chain: rpc_client::Chain::Bitcoin,
        url: "https://esplora.testnet.example".into(),
        network: wallet_domain::account::Network::Testnet,
        policy: ProviderPolicy::dev_default(rpc_client::Chain::Bitcoin),
    };
    BitcoinConfig {
        network: wallet_domain::account::Network::Testnet,
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
