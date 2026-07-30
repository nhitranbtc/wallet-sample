//! Transport-level tests for [`SolanaAdapter`].
//!
//! No real Solana JSON-RPC transport is exercised here — the proof
//! substitutes the validation surface that
//! `rpc_client::EndpointConfig::validate` already enforces:
//!
//! 1. HTTPS-only URLs,
//! 2. `Network::Mainnet` is forbidden by the architecture proof,
//! 3. hosts must appear in the provider-policy allow-list.
//!
//! These three properties are the same ones the Bitcoin and
//! Ethereum adapters pin in their `mock_transport.rs`, because the
//! gateway policy is the same `rpc_client::EndpointConfig`
//! regardless of chain.

use chain_solana::{SolanaAdapter, SolanaConfig};
use rpc_client::{Chain, EndpointConfig, Network, ProviderPolicy};
use wallet_domain::account::Network as Net;
use wallet_domain::error::ErrorCategory;

#[tokio::test]
async fn construction_rejects_plaintext_endpoint() {
    let config = SolanaConfig {
        network: Net::Testnet,
        endpoint: EndpointConfig {
            chain: Chain::Solana,
            url: "http://api.devnet.solana.com".into(),
            network: Network::Testnet,
            policy: ProviderPolicy::dev_default(Chain::Solana),
        },
        rpc_url: "http://api.devnet.solana.com".into(),
    };

    let err = SolanaAdapter::new(config).unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Configuration);
}

#[tokio::test]
async fn construction_rejects_off_policy_host() {
    let config = SolanaConfig {
        network: Net::Testnet,
        endpoint: EndpointConfig {
            chain: Chain::Solana,
            url: "https://api.malicious.example".into(),
            network: Network::Testnet,
            policy: ProviderPolicy::dev_default(Chain::Solana),
        },
        rpc_url: "https://api.malicious.example".into(),
    };

    let err = SolanaAdapter::new(config).unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Configuration);
}