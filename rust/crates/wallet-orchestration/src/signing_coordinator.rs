use crate::session::SessionState;
use secure_storage::{BiometricProof, KeyPurpose};
use std::sync::atomic::{AtomicBool, Ordering};
use wallet_domain::error::WalletError;

/// Coordinator that consumes a `BiometricProof` from `secure-storage`
/// and turns it into authorization to sign on behalf of the active
/// session.
///
/// There is no synthetic-grant path: only `BiometricProof::Granted`
/// with `KeyPurpose::Sign` is accepted. A second call to `consume`
/// after a successful grant is rejected with `WalletError::Authentication`.
pub struct SigningCoordinator {
    /// Single-consume guard. Set to `true` after the first successful
    /// `consume`; subsequent calls fail until reset.
    consumed: AtomicBool,
}

impl SigningCoordinator {
    pub fn new() -> Self {
        Self {
            consumed: AtomicBool::new(false),
        }
    }

    /// Consume a `BiometricProof`. Returns `Ok(())` only if:
    /// - the proof is `Granted { purpose: KeyPurpose::Sign, .. }`, and
    /// - the session is in the `Ready` phase, and
    /// - no prior `consume` call has succeeded on this coordinator.
    ///
    /// All other paths return an error and leave the coordinator's
    /// consumed flag unchanged.
    pub fn consume(
        &self,
        session: &mut SessionState,
        proof: BiometricProof,
    ) -> Result<(), WalletError> {
        match proof {
            BiometricProof::Granted {
                purpose: KeyPurpose::Sign,
                ..
            } => {
                if !session.is_ready() {
                    return Err(WalletError::Locked);
                }
                // Single-consume semantics: `compare_exchange` enforces
                // that exactly one successful call wins. The losing
                // caller is reported as an authentication failure so
                // a re-issued proof cannot bypass the gate.
                if self
                    .consumed
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_err()
                {
                    return Err(WalletError::Authentication(
                        "signing proof already consumed".into(),
                    ));
                }
                Ok(())
            }
            _ => Err(WalletError::Authentication(
                "proof was not for Sign purpose".into(),
            )),
        }
    }

    /// Rejects any synthetic grant type. Exists solely to make the
    /// API prove that no path other than `secure_storage::BiometricProof`
    /// can bypass the gate. The `_t` parameter accepts any type and is
    /// dropped without effect.
    pub fn try_with_proof<T>(
        &self,
        _session: &mut SessionState,
        _t: T,
    ) -> Result<(), WalletError> {
        Err(WalletError::Authentication(
            "only secure-storage::BiometricProof is accepted".into(),
        ))
    }
}

impl Default for SigningCoordinator {
    fn default() -> Self {
        Self::new()
    }
}