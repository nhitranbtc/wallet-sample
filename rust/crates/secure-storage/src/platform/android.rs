use crate::error::SecureStorageError;
use crate::platform::{BiometricProof, KeyPurpose, SecureStorage, WrappedKey};
use async_trait::async_trait;

pub struct AndroidSecureStorage {
    pub key_alias_prefix: String,
}

#[async_trait]
impl SecureStorage for AndroidSecureStorage {
    async fn wrap_key(&self, _dek: &[u8; 32]) -> Result<WrappedKey, SecureStorageError> {
        // Requires on-device execution through Android Keystore or StrongBox.
        unimplemented!("Android Keystore wrapping requires on-device execution")
    }

    async fn unwrap_key(
        &self,
        _wrapped: &WrappedKey,
        _purpose: KeyPurpose,
        _challenge: [u8; 32],
    ) -> Result<BiometricProof, SecureStorageError> {
        // Requires on-device execution through BiometricPrompt and Android Keystore.
        unimplemented!("Android Keystore unwrap requires on-device execution")
    }

    async fn is_hardware_backed(&self) -> bool {
        #[cfg(target_os = "android")]
        {
            true
        }
        #[cfg(not(target_os = "android"))]
        {
            false
        }
    }
}
