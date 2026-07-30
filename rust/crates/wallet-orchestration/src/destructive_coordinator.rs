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
