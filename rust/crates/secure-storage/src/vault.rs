use crate::error::SecureStorageError;
use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::Rng;
use zeroize::Zeroizing;

pub const VAULT_VERSION: u32 = 1;

pub struct Vault {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    version: u32,
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
        let associated_data = version.to_le_bytes();
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
        })
    }

    pub fn decrypt(&self, dek: &[u8; 32]) -> Result<Zeroizing<Vec<u8>>, SecureStorageError> {
        Self::decrypt_with_version(dek, &self.nonce, &self.ciphertext, self.version)
    }

    pub(crate) fn decrypt_with_version(
        dek: &[u8; 32],
        nonce: &[u8; 12],
        ciphertext: &[u8],
        version: u32,
    ) -> Result<Zeroizing<Vec<u8>>, SecureStorageError> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(dek));
        let associated_data = version.to_le_bytes();
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(nonce),
                Payload {
                    msg: ciphertext,
                    aad: &associated_data,
                },
            )
            .map_err(|_| SecureStorageError::Integrity)?;

        Ok(Zeroizing::new(plaintext))
    }

    /// Encrypt `plaintext` under `dek` and return a self-contained, sealed,
    /// version-bound ciphertext blob. Pure serialization; no `Vault` instance
    /// is kept by the caller.
    pub fn seal(dek: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
        let nonce: [u8; 12] = rand::thread_rng().gen();
        let ciphertext = Self::encrypt(dek, &nonce, plaintext)?;
        let mut out = Vec::with_capacity(4 + 12 + ciphertext.ciphertext.len());
        out.extend_from_slice(&VAULT_VERSION.to_le_bytes());
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ciphertext.ciphertext);
        Ok(out)
    }

    /// Reverse of `seal`. Returns the plaintext in a `Zeroizing<Vec<u8>>` so
    /// it clears on drop.
    pub fn open(dek: &[u8; 32], blob: &[u8]) -> Result<Zeroizing<Vec<u8>>, SecureStorageError> {
        if blob.len() < 4 + 12 {
            return Err(SecureStorageError::Integrity);
        }
        let version = u32::from_le_bytes(
            blob[..4]
                .try_into()
                .map_err(|_| SecureStorageError::Integrity)?,
        );
        let nonce: [u8; 12] = blob[4..16]
            .try_into()
            .map_err(|_| SecureStorageError::Integrity)?;
        let ct = &blob[16..];
        // Validate the embedded version by binding it as AAD; an AAD
        // mismatch makes AES-GCM fail authentication, which surfaces as
        // `SecureStorageError::Integrity`.
        Self::decrypt_with_version(dek, &nonce, ct, version)
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
