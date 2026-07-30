//! Derivation vectors tying the keystore's Solana key path to
//! `solana_signer`'s BIP-44 ed25519 derivation machinery.
//!
//! Placeholder. The real vectors (BIP-39 test mnemonic -> `m/44'/501'/0'/0/0` -> base58
//! Solana address, cross-checked against `solana_signer::Signer`)
//! land once Task 8's bridge implementation wires the
//! `solana_rpc_client` transport, and Task 10 (`wallet-orchestration`) completes
//! the signer. Marked `#[ignore]` so the suite stays honest about
//! what is not yet covered.

#[test]
#[ignore = "derivation vectors land once Task 8 wires the solana_rpc_client transport + solana_signer"]
fn sol_account_zero_matches_published_vector() {}