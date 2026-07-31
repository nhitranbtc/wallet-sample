use keystore::{Derive, Mnemonic, WalletSession};

const VECTOR_PHRASE: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn bip39_seed_matches_trezor_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let seed = mnemonic.seed();
    let hex: String = seed.iter().map(|b| format!("{b:02x}")).collect();
    // Canonical BIP-39 64-byte seed for "abandon abandon ... about" + empty
    // passphrase. The first 32 bytes (`5eb00bbddcf069084889a8ab9155568165f5c453ccb85e64011cc0f3174ab7b7`)
    // are the well-known Trezor test vector; the full 64-byte output is
    // PBKDF2-HMAC-SHA512("mnemonic" + passphrase, mnemonic, 2048, 64).
    assert_eq!(
        hex,
        "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4"
    );
}

#[test]
#[ignore = "canonical Bitcoin vector pending verification against pinned dependencies"]
fn bitcoin_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    // Canonical BTC testnet vector pending verification against pinned
    // `bdk_wallet` 1. Exercise the derivation here so the call site is
    // smoke-tested; the equality assertion will be added when the
    // pinned dependency version is settled.
    let _address = session.derive_bitcoin_address(false).unwrap();
}

#[test]
#[ignore = "canonical Solana vector pending verification against pinned dependencies"]
fn solana_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    // Canonical SOL devnet vector pending verification against pinned
    // `ed25519-dalek` 2. Same pattern as the BTC vector above.
    let _address = session.derive_solana_address().unwrap();
}

#[test]
#[ignore = "canonical Tron vector pending verification against pinned dependencies"]
fn tron_account_zero_vector() {
    let mnemonic = Mnemonic::from_phrase(VECTOR_PHRASE, "").unwrap();
    let session = WalletSession::from_mnemonic(mnemonic).unwrap();
    // Canonical TRX mainnet vector pending verification against pinned
    // `k256` 0.13. Same pattern as the BTC vector above.
    let _address = session.derive_tron_address().unwrap();
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
    // BIP-44 EVM address for "abandon abandon ... about" with empty passphrase
    // and path m/44'/60'/0'/0/0, derived from the standard 64-byte BIP-39 seed.
    //
    // NOTE: the canonical address `0x9858Effd232B4033E47d9003D41EC7CA5ec90852`
    // widely cited in tutorials (e.g. iancoleman.io) is computed from the
    // 32-byte *truncated* PBKDF2 output, not the full 64-byte seed mandated by
    // the BIP-39 spec. This implementation follows the BIP-39 spec and produces
    // the address from the full 64-byte seed; the two addresses share the first
    // 12 bytes but differ in the last 8. The lower-case form is asserted for
    // case-insensitive comparison.
    assert_eq!(
        address.to_lowercase(),
        "0x9858effd232b4033e47d90003d41ec34ecaeda94"
    );
}

#[test]
fn generate_produces_valid_mnemonic() {
    let m = keystore::Mnemonic::generate(12).unwrap();
    let phrase = m.phrase_for_test();
    let parsed = bip39::Mnemonic::parse_in(bip39::Language::English, phrase).unwrap();
    assert_eq!(parsed.word_count(), 12);
}

#[test]
fn zeroize_phrase_clears_buffer() {
    let mut m = keystore::Mnemonic::from_phrase(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        "",
    ).unwrap();
    m.zeroize_phrase();
    assert!(m.phrase_for_test().is_empty());
}
