use keystore::{Derive, Mnemonic, WalletSession};

const VECTOR_PHRASE: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn bip39_seed_matches_trezor_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let seed = mnemonic.seed();
    let hex: String = seed.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        hex,
        "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e64011cc0f3174ab7b7"
    );
}

#[test]
#[ignore = "canonical Bitcoin vector pending verification against pinned dependencies"]
fn bitcoin_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    assert_eq!(
        session.derive_bitcoin_address(false).unwrap(),
        unimplemented!("BTC canonical vector pending verification against pinned bdk_wallet 1")
    );
}

#[test]
#[ignore = "canonical Solana vector pending verification against pinned dependencies"]
fn solana_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    assert_eq!(
        session.derive_solana_address().unwrap(),
        unimplemented!("SOL canonical vector pending verification against pinned ed25519-dalek 2")
    );
}

#[test]
#[ignore = "canonical Tron vector pending verification against pinned dependencies"]
fn tron_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    assert_eq!(
        session.derive_tron_address().unwrap(),
        unimplemented!("TRX canonical vector pending verification against pinned k256 0.13")
    );
}

#[test]
fn evm_account_zero_matches_known_address() {
    let mnemonic = keystore::Mnemonic::from_phrase(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        "",
    )
    .unwrap();
    let session = keystore::WalletSession::from_mnemonic(mnemonic).unwrap();
    let address = session.derive_evm_address().unwrap();
    assert_eq!(
        address.to_lowercase(),
        "0x9858effd232b4033e47d9003d41ec7ca5ec90852"
    );
}
