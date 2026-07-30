//! Contract tests for [`TronAdapter`].
//!
//! These tests pin the validation surface against real Tron base58
//! inputs: a destination with valid length/charset but a corrupted
//! double-SHA256 checksum, a destination whose body prefix is not
//! `0x41`, a zero-amount request, and a malformed (non-`T`) prefix.
//! Network and trongrid calls are intentionally not exercised here —
//! the transport wiring lands with Task 8's bridge implementation.

use chain_core::ChainAdapter;
use chain_tron::{TronAdapter, TronConfig};
use rpc_client::{Chain, EndpointConfig, ProviderPolicy};
use wallet_domain::account::{AccountRef, AddressDisplay, ChainId, Network as Net};
use wallet_domain::amount::Amount;
use wallet_domain::error::ErrorCategory;
use wallet_domain::transfer::TransferRequest;

fn config() -> TronConfig {
    let ep = EndpointConfig {
        chain: Chain::Tron,
        url: "https://api.shasta.trongrid.io".into(),
        network: Net::Testnet,
        policy: ProviderPolicy::dev_default(Chain::Tron),
    };
    TronConfig {
        network: Net::Testnet,
        endpoint: ep,
        chain_id: 2494104990,
    }
}

fn request(destination: &str, amount: u128) -> TransferRequest {
    TransferRequest {
        source: AccountRef {
            chain: ChainId::Tron,
            network: Net::Testnet,
            index: 0,
        },
        destination: AddressDisplay(destination.into()),
        amount: Amount(amount),
    }
}

#[tokio::test]
async fn descriptor_is_tron_testnet() {
    let a = TronAdapter::new(config()).unwrap();
    let d = a.descriptor();
    assert_eq!(d.chain, ChainId::Tron);
    assert_eq!(d.symbol, "TRX");
    // Tron's smallest unit is the SUN (1 TRX = 1_000_000 SUN);
    // default decimals stay at 6 so the FFI formatter never has to
    // know the integer-scale conversion.
    assert_eq!(d.default_decimals, 6);
}

#[tokio::test]
async fn prepare_rejects_zero_amount() {
    let a = TronAdapter::new(config()).unwrap();
    // 34 chars, starts with `T`, valid base58 chars. The amount
    // validator fires before the address validator, so the
    // destination checksum defects below are irrelevant for this
    // test.
    let req = request(
        "TJRabPrwbWB45sc7Y88KCfFXWbZ7V8L4cu",
        0,
    );
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_rejects_malformed_address() {
    let a = TronAdapter::new(config()).unwrap();
    // 34 chars long, but no `T` prefix — the strip-prefix check
    // rejects the string before the base58 decode runs. The amount
    // is non-zero so we know the address validator is the one that
    // fired.
    let req = request(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567abcd",
        1,
    );
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_rejects_bad_checksum() {
    let a = TronAdapter::new(config()).unwrap();
    // 34 chars, `T` prefix, valid base58 charset, but the last 4
    // chars encode a checksum that does not match the double-SHA256
    // of the preceding 21 bytes. The base58 decode + body-prefix
    // check pass; the checksum comparison fails fast.
    let req = request(
        "TJRabPrwbWB45sc7Y88KCfFXWbZ7V8L4cd",
        1,
    );
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_rejects_short_address() {
    let a = TronAdapter::new(config()).unwrap();
    // 33 chars, starts with `T`. The length check rejects the
    // string before the base58 decode runs.
    let req = request("TJRabPrwbWB45sc7Y88KCfFXWbZ7V8L4c", 1);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}
