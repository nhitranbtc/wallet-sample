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
    pub fn try_with_proof<T>(&self, _session: &mut SessionState, _t: T) -> Result<(), WalletError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::destructive_coordinator::DestructiveCoordinator;
    use keystore::{Mnemonic, WalletSession};
    use secure_storage::WrappedKey;

    const TREZOR_PHRASE: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn ready_session() -> SessionState {
        // Mnemonic is intentionally non-Clone and consumed by both APIs,
        // so derive two equivalent values from the deterministic test vector.
        let mnemonic_for_enroll =
            Mnemonic::from_phrase(TREZOR_PHRASE, "").expect("Trezor test mnemonic parses");
        let mnemonic_for_session =
            Mnemonic::from_phrase(TREZOR_PHRASE, "").expect("Trezor test mnemonic parses");
        let wallet_session =
            WalletSession::from_mnemonic(mnemonic_for_session).expect("session from mnemonic");

        let mut session = SessionState::new();
        session
            .begin_enroll(mnemonic_for_enroll)
            .expect("begin enrollment");
        session.activate(wallet_session).expect("activate session");
        session
    }

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
    fn coordinator_rejects_double_consume() {
        let coordinator = SigningCoordinator::new();
        let mut session = ready_session();
        coordinator
            .consume(&mut session, proof_with(KeyPurpose::Sign))
            .unwrap();
        let err = coordinator
            .consume(&mut session, proof_with(KeyPurpose::Sign))
            .unwrap_err();
        assert!(matches!(err, WalletError::Authentication(_)));
    }

    #[test]
    fn coordinator_rejects_locked_session() {
        let coordinator = SigningCoordinator::new();
        let mut session = ready_session();
        session.lock();
        let err = coordinator
            .consume(&mut session, proof_with(KeyPurpose::Sign))
            .unwrap_err();
        assert!(matches!(err, WalletError::Locked));
    }

    #[test]
    fn coordinator_rejects_synthetic_proof() {
        let coordinator = SigningCoordinator::new();
        let mut session = ready_session();
        // No synthetic BiometricProof can be constructed outside secure-storage.
        let result = coordinator.try_with_proof(&mut session, std::marker::PhantomData::<()>);
        assert!(matches!(result, Err(WalletError::Authentication(_))));
    }

    #[test]
    fn lock_is_noop_on_removed_session() {
        let mut session = ready_session();
        session.remove();
        assert!(session.is_removed(), "precondition: phase is Removed");
        session.lock();
        assert!(session.is_removed(), "lock() must be a no-op on Removed");
    }

    #[test]
    fn lock_wallet_with_sign_purpose_is_rejected() {
        let mut session = ready_session();
        let err = DestructiveCoordinator::lock_wallet(&mut session, proof_with(KeyPurpose::Sign))
            .unwrap_err();
        assert!(matches!(err, WalletError::Authentication(_)));
        assert!(session.is_ready(), "rejected lock must not change phase");
    }

    // Positive control: without this, the rejection test would still pass
    // if lock_wallet simply rejected every proof.
    #[test]
    fn lock_wallet_with_lock_purpose_is_accepted() {
        let mut session = ready_session();
        DestructiveCoordinator::lock_wallet(&mut session, proof_with(KeyPurpose::Lock)).unwrap();
        assert!(!session.is_ready(), "accepted lock must leave Ready");
    }

    #[test]
    fn lock_wallet_with_wipe_purpose_is_rejected() {
        let mut session = ready_session();
        let err = DestructiveCoordinator::lock_wallet(&mut session, proof_with(KeyPurpose::Wipe))
            .unwrap_err();
        assert!(matches!(err, WalletError::Authentication(_)));
    }
}
