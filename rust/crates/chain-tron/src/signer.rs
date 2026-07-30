//! Tron secp256k1 signer wiring.
//!
//! The on-device signing path lives in **Task 10
//! (`wallet-orchestration`)**, which owns the unlocked
//! [`keystore::WalletSession`] — the only layer allowed to hold
//! secret material. Task 10 completes this function by feeding the
//! derived Tron BIP-44 secret (path `m/44'/195'/0'/0/0`) into the
//! signing machinery and signing the prepared
//! `PreparedPayload::Tron { ref_block_bytes, ref_block_hash }`. Until
//! then the call site does not exist and this body is intentionally
//! `unimplemented!()` rather than a silently-wrong stub signer.

use keystore::Derive;
use wallet_domain::error::WalletError;

/// Returns the derived Tron secp256k1 public key bytes.
///
/// # Task 10 boundary
///
/// Task 10 (`wallet-orchestration`) replaces the `unimplemented!()`
/// below with real Tron signing plumbing over the derived
/// secp256k1 secret. The function is left here, called from
/// `adapter.rs`'s prepare/broadcast paths, only so the boundary is
/// visible at compile time and so the test suite can still link the
/// crate without dragging in the heavy signing machinery for the
/// architecture proof.
pub fn build_signer(session: &impl Derive) -> Result<[u8; 33], WalletError> {
    let _key = session.derive_tron_key()?;
    unimplemented!(
        "Task 10 (wallet-orchestration) wires the Tron secp256k1 signer over the derived Tron secret"
    )
}
