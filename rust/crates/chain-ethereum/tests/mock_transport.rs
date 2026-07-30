//! Transport-level tests for the Ethereum adapter.
//!
//! The mocked-RPC harness (an alloy provider backed by a canned
//! transport, exercising `eth_getBalance` / `eth_gasPrice` /
//! `eth_sendRawTransaction`) lands with Task 8's bridge implementation,
//! once `synchronize` / `prepare_transfer` / `broadcast` stop returning
//! placeholder values. Until then this file pins the one transport
//! property that is already enforced: an adapter cannot be constructed
//! over an endpoint that violates the provider policy.

use chain_ethereum::{EthereumAdapter, EthereumConfig};
use rpc_client::{Chain, EndpointConfig, Network, ProviderPolicy};
use wallet_domain::account::Network as Net;
use wallet_domain::error::ErrorCategory;

#[tokio::test]
async fn construction_rejects_plaintext_endpoint() {
    let config = EthereumConfig {
        network: Net::Testnet,
        endpoint: EndpointConfig {
            chain: Chain::Ethereum,
            url: "http://rpc.sepolia.org".into(),
            network: Network::Testnet,
            policy: ProviderPolicy::dev_default(Chain::Ethereum),
        },
        chain_id: 11155111,
    };

    let err = EthereumAdapter::new(config).unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Configuration);
}
