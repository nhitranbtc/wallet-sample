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
    assert_eq!(session.derive_bitcoin_address(false).unwrap(), "TODO");
}

#[test]
#[ignore = "canonical Solana vector pending verification against pinned dependencies"]
fn solana_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    assert_eq!(session.derive_solana_address().unwrap(), "TODO");
}

#[test]
#[ignore = "canonical Tron vector pending verification against pinned dependencies"]
fn tron_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    assert_eq!(session.derive_tron_address().unwrap(), "TODO");
}
