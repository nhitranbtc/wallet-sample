pub mod error;
pub mod platform;
pub mod vault;

pub use error::SecureStorageError;
pub use platform::{
    AndroidSecureStorage, BiometricProof, IosSecureStorage, KeyPurpose, SecureStorage, WrappedKey,
};
pub use vault::Vault;
