//! Derivation vectors tying the keystore's Bitcoin key path to
//! `bdk_wallet`'s BIP-84 descriptor machinery.
//!
//! Placeholder. The real vectors (BIP-39 test mnemonic -> `m/84'/1'/0'/0` -> bech32
//! testnet address, cross-checked against `bdk_wallet::descriptor`)
//! land once Task 8's bridge implementation wires the descriptor +
//! Esplora transport, and Task 10 (`wallet-orchestration`) completes
//! the signer. Marked `#[ignore]` so the suite stays honest about
//! what is not yet covered.

#[test]
#[ignore = "derivation vectors land once Task 8 wires the BDK descriptor + Esplora transport"]
fn btc_account_zero_matches_published_vector() {}
