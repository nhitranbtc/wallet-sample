//! BDK signer wiring.
//!
//! The on-device signing path lives in **Task 10
//! (`wallet-orchestration`)**, which owns the unlocked
//! [`keystore::WalletSession`] — the only layer allowed to hold
//! secret material. Task 10 completes this function by feeding the
//! derived BIP-84 secret into `bdk_wallet`'s signer machinery
//! (`Descriptor::derive` -> `Signer` -> PSBT signing). Until then
//! the call site does not exist and this body is intentionally
//! `unimplemented!()` rather than a silently-wrong stub signer.

use keystore::Derive;
use wallet_domain::error::WalletError;

/// Builds a BDK signer from the derived account-zero Bitcoin key.
///
/// # Task 10 boundary
///
/// Task 10 (`wallet-orchestration`) replaces the `unimplemented!()`
/// below with real `bdk_wallet` signing plumbing. The function is
/// left here, called from `adapter.rs`'s prepare/broadcast paths,
/// only so the boundary is visible at compile time and so the test
/// suite can still link the crate without dragging in
/// `bdk_wallet`'s heavy `SignOptions` machinery for the architecture
/// proof.
pub fn build_signer(session: &impl Derive) -> Result<bdk_wallet::KeychainKind, WalletError> {
    let _sk = session.derive_bitcoin_key()?;
    unimplemented!(
        "Task 10 (wallet-orchestration) wires BDK signer over the derived Bitcoin secret"
    )
}
