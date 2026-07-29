use crate::error::SecureStorageError;
use crate::platform::{BiometricProof, KeyPurpose, SecureStorage, WrappedKey};
use async_trait::async_trait;

pub struct IosSecureStorage {
    pub service: String,
}

#[async_trait]
impl SecureStorage for IosSecureStorage {
    async fn wrap_key(&self, _dek: &[u8; 32]) -> Result<WrappedKey, SecureStorageError> {
        // Requires on-device execution through the iOS Keychain and Secure Enclave.
        unimplemented!("iOS Keychain wrapping requires on-device execution")
    }

    async fn unwrap_key(
        &self,
        _wrapped: &WrappedKey,
        _purpose: KeyPurpose,
        _challenge: [u8; 32],
    ) -> Result<BiometricProof, SecureStorageError> {
        // Requires on-device execution through LocalAuthentication and the iOS Keychain.
        unimplemented!("iOS Keychain unwrap requires on-device execution")
    }

    async fn is_hardware_backed(&self) -> bool {
        #[cfg(target_os = "ios")]
        {
            true
        }
        #[cfg(not(target_os = "ios"))]
        {
            false
        }
    }
}
