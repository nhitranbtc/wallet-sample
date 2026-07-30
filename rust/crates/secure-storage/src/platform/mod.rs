use crate::error::SecureStorageError;
use async_trait::async_trait;

pub mod android;
pub mod ios;

pub use android::AndroidSecureStorage;
pub use ios::IosSecureStorage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedKey {
    pub reference: String,
    pub algorithm: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPurpose {
    Sign,
    Decrypt,
    Wipe,
}

/// Constructible only by platform secure-storage implementations that have
/// completed a fresh OS biometric / device-auth challenge for `purpose`.
/// `wallet-orchestration::signing_coordinator::consume` accepts this type
/// and nothing else — a synthetic `BiometricProof::Granted` cannot be
/// constructed by Flutter or by Dart-side code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiometricProof {
    #[non_exhaustive]
    Granted {
        wrapped: WrappedKey,
        purpose: KeyPurpose,
        challenge: [u8; 32],
    },
}

impl BiometricProof {
    /// Test-only constructor: builds a `Granted` variant directly.
    /// Always available so integration tests in `tests/` can call it
    /// (Cargo does not propagate `cfg(test)` from the test target back
    /// into the library it depends on); the `_for_test` suffix and
    /// `#[doc(hidden)]` mark it as not part of the production API.
    /// Production code paths must route through a real platform
    /// secure-storage implementation.
    #[doc(hidden)]
    pub fn granted_for_test(
        wrapped: WrappedKey,
        purpose: KeyPurpose,
        challenge: [u8; 32],
    ) -> Self {
        Self::Granted {
            wrapped,
            purpose,
            challenge,
        }
    }
}

#[async_trait]
pub trait SecureStorage: Send + Sync {
    async fn wrap_key(&self, dek: &[u8; 32]) -> Result<WrappedKey, SecureStorageError>;

    async fn unwrap_key(
        &self,
        wrapped: &WrappedKey,
        purpose: KeyPurpose,
        challenge: [u8; 32],
    ) -> Result<BiometricProof, SecureStorageError>;

    async fn is_hardware_backed(&self) -> bool;
}
