use crate::session::SessionState;
use secure_storage::{BiometricProof, KeyPurpose};
use wallet_domain::error::WalletError;

/// Coordinator for destructive wallet operations: full remove and
/// session lock. Both require a fresh `BiometricProof` from
/// `secure-storage` — `Wipe` purpose for removal, `Sign` purpose for
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

    /// Lock the active session. Requires a `Sign`-purpose proof.
    pub fn lock_wallet(
        session: &mut SessionState,
        proof: BiometricProof,
    ) -> Result<(), WalletError> {
        match proof {
            BiometricProof::Granted {
                purpose: KeyPurpose::Sign,
                ..
            } => {
                session.lock();
                Ok(())
            }
            _ => Err(WalletError::Authentication(
                "lock requires Sign-purpose proof".into(),
            )),
        }
    }
}