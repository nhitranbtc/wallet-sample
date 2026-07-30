//! Derivation vectors tying the keystore's EVM key path to alloy.
//!
//! Placeholder. The real vectors (BIP-39 test mnemonic ->
//! `m/44'/60'/0'/0/0` -> checksummed address, cross-checked against
//! `alloy::primitives::Address`) land with Task 8's bridge
//! implementation, alongside the signer wiring that Task 10
//! (`wallet-orchestration`) completes. Marked `#[ignore]` so the suite
//! stays honest about what is not yet covered.

#[test]
#[ignore = "derivation vectors land with the Task 8 bridge implementation"]
fn evm_account_zero_matches_published_vector() {}
