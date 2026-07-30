//! Transport-level tests for [`BitcoinAdapter`].
//!
//! No real Esplora HTTP transport is exercised here — the proof
//! substitutes the validation surface that
//! `rpc_client::EndpointConfig::validate` already enforces:
//!
//! 1. HTTPS-only URLs,
//! 2. `Network::Mainnet` is forbidden by the architecture proof,
//! 3. hosts must appear in the provider-policy allow-list.
//!
//! These three properties are the same ones the Ethereum adapter
//! pins in its `mock_transport.rs`, because the gateway policy is
//! the same `rpc_client::EndpointConfig` regardless of chain.

use bdk_wallet::bitcoin::Network as BdkNet;
use chain_bitcoin::{BitcoinAdapter, BitcoinConfig};
use rpc_client::{Chain, EndpointConfig, Network, ProviderPolicy};
use wallet_domain::account::Network as Net;
use wallet_domain::error::ErrorCategory;

#[tokio::test]
async fn construction_rejects_plaintext_endpoint() {
    let config = BitcoinConfig {
        network: Net::Testnet,
        endpoint: EndpointConfig {
            chain: Chain::Bitcoin,
            url: "http://esplora.testnet.example".into(),
            network: Network::Testnet,
            policy: ProviderPolicy::dev_default(Chain::Bitcoin),
        },
        bdk_network: BdkNet::Testnet,
        encrypted_db_path: "/tmp/chain-bitcoin-plaintext-test.sqlite".into(),
    };

    let err = BitcoinAdapter::new(config).unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Configuration);
}

#[tokio::test]
async fn construction_rejects_off_policy_host() {
    let config = BitcoinConfig {
        network: Net::Testnet,
        endpoint: EndpointConfig {
            chain: Chain::Bitcoin,
            url: "https://esplora.malicious.example".into(),
            network: Network::Testnet,
            policy: ProviderPolicy::dev_default(Chain::Bitcoin),
        },
        bdk_network: BdkNet::Testnet,
        encrypted_db_path: "/tmp/chain-bitcoin-offpolicy-test.sqlite".into(),
    };

    let err = BitcoinAdapter::new(config).unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Configuration);
}
