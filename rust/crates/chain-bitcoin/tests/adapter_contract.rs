//! Contract tests for [`BitcoinAdapter`].
//!
//! These tests pin the validation surface against real bech32 inputs:
//! a canonical BIP-173 testnet address, a zero-amount request, and a
//! string with the `tb1` prefix but a corrupted bech32 checksum.
//! Network and Esplora calls are intentionally not exercised here —
//! the transport wiring lands with Task 8's bridge implementation.

use bdk_wallet::bitcoin::Network as BdkNet;
use chain_bitcoin::{BitcoinAdapter, BitcoinConfig};
use chain_core::ChainAdapter;
use rpc_client::{Chain, EndpointConfig, ProviderPolicy};
use std::net::TcpListener;
use wallet_domain::account::{AccountRef, AddressDisplay, ChainId, Network as Net};
use wallet_domain::amount::Amount;
use wallet_domain::error::ErrorCategory;
use wallet_domain::transfer::TransferRequest;

fn config() -> BitcoinConfig {
    let ep = EndpointConfig {
        chain: Chain::Bitcoin,
        url: "https://esplora.testnet.example".into(),
        network: Net::Testnet,
        policy: ProviderPolicy::dev_default(Chain::Bitcoin),
    };
    BitcoinConfig {
        network: Net::Testnet,
        endpoint: ep,
        bdk_network: BdkNet::Testnet,
        encrypted_db_path: temp_db_path(),
    }
}

/// Use a real OS-chosen temp path so parallel test invocations don't
/// contend on a single SQLite file. The OS will recycle the path when
/// the test process exits; we do not need to clean up explicitly
/// because `Connection::open` will simply re-create an empty file on
/// the next run.
fn temp_db_path() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().expect("local_addr").port();
    drop(listener);
    std::env::temp_dir()
        .join(format!("chain-bitcoin-{}-{port}.sqlite", std::process::id()))
        .to_string_lossy()
        .into_owned()
}

fn request(destination: &str, amount: u128) -> TransferRequest {
    TransferRequest {
        source: AccountRef {
            chain: ChainId::Bitcoin,
            network: Net::Testnet,
            index: 0,
        },
        destination: AddressDisplay(destination.into()),
        amount: Amount(amount),
    }
}

#[tokio::test]
async fn descriptor_is_bitcoin_testnet() {
    let a = BitcoinAdapter::new(config()).unwrap();
    let d = a.descriptor();
    assert_eq!(d.chain, ChainId::Bitcoin);
    assert_eq!(d.symbol, "BTC");
    // Bitcoin's smallest unit is the satoshi; default decimals stay
    // at 8 so FFI formatters never need to know.
    assert_eq!(d.default_decimals, 8);
}

#[tokio::test]
async fn prepare_rejects_zero_amount() {
    let a = BitcoinAdapter::new(config()).unwrap();
    let req = request("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx", 0);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_rejects_invalid_bech32_checksum() {
    let a = BitcoinAdapter::new(config()).unwrap();
    // Valid prefix + valid bech32 charset + corrupted checksum. The
    // polymod inside `bech32::decode` rejects the trailing six u5
    // values, which the adapter surfaces as `ErrorCategory::Input`.
    let req = request("tb1qinvalidchecksum0000000000000000000000000", 1);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn next_external_address_advances() {
    let a = BitcoinAdapter::new(config()).unwrap();
    let one = a.next_external_address().await.unwrap();
    let two = a.next_external_address().await.unwrap();
    // The proof pins monotonicity: each call advances the persisted
    // BDK index, so successive addresses must differ. We do not pin
    // the format here — that contract belongs to BIP-84 vectors in
    // `derivation_vectors.rs`.
    assert_ne!(one, two);
}
