use crate::error::SecureStorageError;
use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use zeroize::Zeroizing;

pub const VAULT_VERSION: u32 = 1;

pub struct Vault {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    version: u32,
    associated_data: Vec<u8>,
}

impl Vault {
    pub fn encrypt(
        dek: &[u8; 32],
        nonce: &[u8; 12],
        plaintext: &[u8],
    ) -> Result<Self, SecureStorageError> {
        Self::encrypt_with_version(dek, nonce, plaintext, VAULT_VERSION)
    }

    pub fn encrypt_with_version(
        dek: &[u8; 32],
        nonce: &[u8; 12],
        plaintext: &[u8],
        version: u32,
    ) -> Result<Self, SecureStorageError> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(dek));
        let associated_data = version.to_le_bytes().to_vec();
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(nonce),
                Payload {
                    msg: plaintext,
                    aad: &associated_data,
                },
            )
            .map_err(|_| SecureStorageError::Integrity)?;

        Ok(Self {
            ciphertext,
            nonce: *nonce,
            version,
            associated_data,
        })
    }

    pub fn decrypt(&self, dek: &[u8; 32]) -> Result<Zeroizing<Vec<u8>>, SecureStorageError> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(dek));
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(&self.nonce),
                Payload {
                    msg: &self.ciphertext,
                    aad: &self.associated_data,
                },
            )
            .map_err(|_| SecureStorageError::Integrity)?;

        Ok(Zeroizing::new(plaintext))
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    #[doc(hidden)]
    pub fn corrupt_for_test(&mut self, offset: usize) {
        if let Some(byte) = self.ciphertext.get_mut(offset) {
            *byte ^= 0xFF;
        }
    }
}
