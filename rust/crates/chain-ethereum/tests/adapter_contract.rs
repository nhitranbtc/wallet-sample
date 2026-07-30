use chain_core::ChainAdapter;
use chain_ethereum::{EthereumAdapter, EthereumConfig};
use rpc_client::{Chain, EndpointConfig, Network, ProviderPolicy};
use wallet_domain::account::{AccountRef, AddressDisplay, ChainId, Network as Net};
use wallet_domain::amount::Amount;
use wallet_domain::error::ErrorCategory;
use wallet_domain::transfer::TransferRequest;

fn config() -> EthereumConfig {
    let ep = EndpointConfig {
        chain: Chain::Ethereum,
        url: "https://rpc.sepolia.org".into(),
        network: Network::Testnet,
        policy: ProviderPolicy::dev_default(Chain::Ethereum),
    };
    EthereumConfig {
        network: Net::Testnet,
        endpoint: ep,
        chain_id: 11155111,
    }
}

fn request(destination: &str, amount: u128) -> TransferRequest {
    TransferRequest {
        source: AccountRef {
            chain: ChainId::Ethereum,
            network: Net::Testnet,
            index: 0,
        },
        destination: AddressDisplay(destination.into()),
        amount: Amount(amount),
    }
}

#[tokio::test]
async fn descriptor_is_ethereum_testnet() {
    let a = EthereumAdapter::new(config()).unwrap();
    let d = a.descriptor();
    assert_eq!(d.chain, ChainId::Ethereum);
    assert_eq!(d.symbol, "ETH");
    assert_eq!(d.default_decimals, 18);
}

#[tokio::test]
async fn prepare_rejects_zero_amount() {
    let a = EthereumAdapter::new(config()).unwrap();
    let req = request("0x0000000000000000000000000000000000000001", 0);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_accepts_lowercase_address() {
    let a = EthereumAdapter::new(config()).unwrap();
    // Lowercase hex is a valid EVM address; EIP-55 checksum casing is
    // optional and must not be required by the adapter.
    let req = request("0xd8da6bf26964af9d7eed9e03e53415d37aa96045", 1);
    assert!(a.prepare_transfer(req).await.is_ok());
}

#[tokio::test]
async fn prepare_rejects_garbage_address() {
    let a = EthereumAdapter::new(config()).unwrap();
    let req = request("not-an-address", 1);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}
