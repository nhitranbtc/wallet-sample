use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SecureStorageError {
    #[error("integrity check failed")]
    Integrity,
    #[error("schema version {found} not supported (need {required})")]
    Unsupported { found: u32, required: u32 },
    #[error("platform keystore unavailable")]
    Unavailable,
    #[error("authentication required but missing")]
    AuthenticationRequired,
    #[error("authentication cancelled")]
    Cancelled,
    #[error("authentication locked out")]
    Lockout,
    #[error("enrollment invalidated")]
    EnrollmentInvalidated,
}
