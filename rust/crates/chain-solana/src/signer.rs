//! Solana ed25519 signer wiring.
//!
//! The on-device signing path lives in **Task 10
//! (`wallet-orchestration`)**, which owns the unlocked
//! [`keystore::WalletSession`] — the only layer allowed to hold
//! secret material. Task 10 completes this function by feeding the
//! derived Solana BIP-44 secret (path `m/44'/501'/0'/0/0`) into
//! `solana_signer::Signer` machinery and signing the prepared
//! `PreparedPayload::Sol { blockhash }`. Until then the call site
//! does not exist and this body is intentionally `unimplemented!()`
//! rather than a silently-wrong stub signer.

use keystore::Derive;
use wallet_domain::error::WalletError;

/// Builds a Solana `solana_signer::Signer` from the derived
/// account-zero ed25519 key.
///
/// # Task 10 boundary
///
/// Task 10 (`wallet-orchestration`) replaces the `unimplemented!()`
/// below with real `solana_signer::Signer` plumbing over the
/// derived Solana ed25519 secret. The function is left here, called
/// from `adapter.rs`'s prepare/broadcast paths, only so the
/// boundary is visible at compile time and so the test suite can
/// still link the crate without dragging in
/// `solana_signer`'s heavy signing machinery for the architecture
/// proof.
pub fn build_signer(session: &impl Derive) -> Result<(), WalletError> {
    let _key = session.derive_solana_key()?;
    unimplemented!(
        "Task 10 (wallet-orchestration) wires solana_signer::Signer over the derived Solana secret"
    )
}