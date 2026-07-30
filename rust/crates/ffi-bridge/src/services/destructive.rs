//! Destructive wallet operations: `lock_wallet` and `remove_wallet`.
//!
//! Both require a fresh [`secure_storage::BiometricProof`] from the
//! platform — `Lock` purpose for lock, `Wipe` purpose for remove.
//! The platform call to fetch the `BiometricProof` is intentionally
//! `unimplemented!()`; this helper only contains the proof-fetch
//! site and the wiring into [`wallet_orchestration::DestructiveCoordinator`].

use crate::error::DartError;
use crate::handle::WalletHandle;

use secure_storage::BiometricProof;
use wallet_domain::error::{ErrorCategory, WalletError};
use wallet_orchestration::DestructiveCoordinator;

/// Lock the active wallet. Requires the session to be `Ready`. The
/// platform call to fetch the `BiometricProof` is intentionally
/// `unimplemented!()`.
pub(crate) fn lock(handle: &WalletHandle) -> Result<(), DartError> {
    // Platform secure-storage call site: fetch the Sign-purpose proof.
    let proof: BiometricProof = unimplemented!(
        "native secure-storage unwrap_key(Sign purpose) for lock_wallet"
    );
    let mut guard = handle.inner().lock().expect("session mutex");
    if !guard.is_ready() {
        return Err(DartError::from(&WalletError::Locked, ErrorCategory::Authorization));
    }
    DestructiveCoordinator::lock_wallet(&mut guard, proof)
        .map_err(|e| DartError::from(&e, ErrorCategory::Authorization))
}

/// Remove the wallet entirely. The platform call to fetch the
/// `BiometricProof` is intentionally `unimplemented!()`.
pub(crate) fn remove(handle: &WalletHandle) -> Result<(), DartError> {
    // Platform secure-storage call site: fetch the Wipe-purpose proof.
    let proof: BiometricProof = unimplemented!(
        "native secure-storage unwrap_key(Wipe purpose) for remove_wallet"
    );
    let mut guard = handle.inner().lock().expect("session mutex");
    DestructiveCoordinator::remove_wallet(&mut guard, proof)
        .map_err(|e| DartError::from(&e, ErrorCategory::Authorization))
}
