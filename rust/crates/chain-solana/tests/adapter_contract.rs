//! Contract tests for [`SolanaAdapter`].
//!
//! These tests pin the validation surface against real Solana
//! pubkey inputs: a canonical ed25519 mainnet address, a
//! zero-amount request, a non-base58 garbage string, and a base58
//! string of the wrong length. Network and RPC calls are
//! intentionally not exercised here — the transport wiring lands
//! with Task 8's bridge implementation and the architecture proof
//! uses a zero-byte blockhash placeholder.

use chain_core::ChainAdapter;
use chain_solana::{SolanaAdapter, SolanaConfig};
use rpc_client::{Chain, EndpointConfig, ProviderPolicy};
use wallet_domain::account::{AccountRef, ChainId, Network as Net};
use wallet_domain::amount::Amount;
use wallet_domain::error::ErrorCategory;
use wallet_domain::transfer::TransferRequest;

fn config() -> SolanaConfig {
    let endpoint = EndpointConfig {
        chain: Chain::Solana,
        url: "https://api.devnet.solana.com".into(),
        network: Net::Testnet,
        policy: ProviderPolicy::dev_default(Chain::Solana),
    };
    SolanaConfig {
        network: Net::Testnet,
        endpoint,
        rpc_url: "https://api.devnet.solana.com".into(),
    }
}

fn request(destination: &str, amount: u128) -> TransferRequest {
    TransferRequest {
        source: AccountRef {
            chain: ChainId::Solana,
            network: Net::Testnet,
            index: 0,
        },
        destination: wallet_domain::account::AddressDisplay(destination.into()),
        amount: Amount(amount),
    }
}

#[tokio::test]
async fn descriptor_is_solana_testnet() {
    let a = SolanaAdapter::new(config()).unwrap();
    let d = a.descriptor();
    assert_eq!(d.chain, ChainId::Solana);
    assert_eq!(d.symbol, "SOL");
    // Solana's smallest unit is the lamport; default decimals stay
    // at 9 so FFI formatters never need to know.
    assert_eq!(d.default_decimals, 9);
}

#[tokio::test]
async fn prepare_rejects_zero_amount() {
    let a = SolanaAdapter::new(config()).unwrap();
    let req = request("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", 0);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_accepts_valid_solana_address() {
    let a = SolanaAdapter::new(config()).unwrap();
    // A known-valid mainnet ed25519 pubkey: the USDC token mint
    // (`EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`) on Solana
    // mainnet. Decoding `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
    // from base58 yields exactly 32 bytes, and the resulting bytes
    // pass the `ed25519_dalek::VerifyingKey::from_bytes` off-curve
    // check (it is a real on-curve ed25519 public key). This is
    // not the System Program (`11111...`) — the System Program
    // decodes to 32 bytes of zero and is rejected by dalek's
    // compressed-point decoder because the all-zero encoding is
    // not a valid Edwards point, even though it is a valid
    // base58-of-32-bytes input.
    let req = request(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        1,
    );
    let prepared = a.prepare_transfer(req).await.expect("valid pubkey");
    assert_eq!(prepared.amount, Amount(1));
}

#[tokio::test]
async fn prepare_rejects_malformed_address() {
    let a = SolanaAdapter::new(config()).unwrap();
    // "not-a-pubkey" is not valid base58 of any length and the
    // bs58 decoder rejects it outright; the adapter surfaces the
    // error as `ErrorCategory::Input` so the FFI layer can hand it
    // back to Dart as a validation error rather than a chain-state
    // failure.
    let req = request("not-a-pubkey", 1);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}

#[tokio::test]
async fn prepare_rejects_wrong_length_base58() {
    let a = SolanaAdapter::new(config()).unwrap();
    // Valid base58 but not 32 bytes — bs58 decode succeeds but
    // the length filter rejects anything other than exactly 32
    // bytes.
    let req = request("2g4SyW", 1);
    let err = a.prepare_transfer(req).await.unwrap_err();
    assert_eq!(err.category(), ErrorCategory::Input);
}