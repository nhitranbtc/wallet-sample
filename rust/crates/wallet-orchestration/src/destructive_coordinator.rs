use crate::session::SessionState;
use secure_storage::{BiometricProof, KeyPurpose};
use wallet_domain::error::WalletError;

/// Coordinator for destructive wallet operations: full remove and
/// session lock. Both require a fresh `BiometricProof` from
/// `secure-storage` — `Wipe` purpose for removal, `Lock` purpose for
/// locking.
pub struct DestructiveCoordinator;

impl DestructiveCoordinator {
    /// Remove the wallet entirely. Requires a `Wipe`-purpose proof.
    pub fn remove_wallet(
        session: &mut SessionState,
        proof: BiometricProof,
    ) -> Result<(), WalletError> {
        match proof {
            BiometricProof::Granted {
                purpose: KeyPurpose::Wipe,
                ..
            } => {
                session.remove();
                Ok(())
            }
            _ => Err(WalletError::Authentication(
                "destructive ops require Wipe-purpose proof".into(),
            )),
        }
    }

    /// Lock the active session. Requires a `Lock`-purpose proof.
    ///
    /// A `Sign`-purpose proof is deliberately rejected: signature
    /// authorization must not double as lock authorization
    /// (post-Task-11 security review, Finding 6).
    pub fn lock_wallet(
        session: &mut SessionState,
        proof: BiometricProof,
    ) -> Result<(), WalletError> {
        match proof {
            BiometricProof::Granted {
                purpose: KeyPurpose::Lock,
                ..
            } => {
                session.lock();
                Ok(())
            }
            _ => Err(WalletError::Authentication(
                "lock requires Lock-purpose proof".into(),
            )),
        }
    }
}

// NOTE: these live here rather than in `tests/signing.rs` because that
// integration test does not currently compile — `SessionState::
// ready_for_test` is gated `#[cfg(test)]`, which is false for the
// `tests/` target. Pre-existing breakage, tracked separately; keeping a
// unit test here means Finding 6 has a gate that actually runs today.
#[cfg(test)]
mod tests {
    use super::*;
    use secure_storage::WrappedKey;

    fn proof_with(purpose: KeyPurpose) -> BiometricProof {
        BiometricProof::granted_for_test(
            WrappedKey {
                reference: "test-ref".to_string(),
                algorithm: "test-alg".to_string(),
            },
            purpose,
            [0u8; 32],
        )
    }

    #[test]
    fn lock_wallet_with_sign_purpose_is_rejected() {
        let mut session = SessionState::ready_for_test();
        let err = DestructiveCoordinator::lock_wallet(&mut session, proof_with(KeyPurpose::Sign))
            .unwrap_err();
        assert!(matches!(err, WalletError::Authentication(_)));
        assert!(session.is_ready(), "rejected lock must not change phase");
    }

    // Positive control: without this, the test above would still pass
    // if `lock_wallet` simply rejected every proof.
    #[test]
    fn lock_wallet_with_lock_purpose_is_accepted() {
        let mut session = SessionState::ready_for_test();
        DestructiveCoordinator::lock_wallet(&mut session, proof_with(KeyPurpose::Lock)).unwrap();
        assert!(!session.is_ready(), "accepted lock must leave Ready");
    }

    #[test]
    fn lock_wallet_with_wipe_purpose_is_rejected() {
        let mut session = SessionState::ready_for_test();
        let err = DestructiveCoordinator::lock_wallet(&mut session, proof_with(KeyPurpose::Wipe))
            .unwrap_err();
        assert!(matches!(err, WalletError::Authentication(_)));
    }
}