use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    Input,
    Configuration,
    Connectivity,
    ChainState,
    Authorization,
    Vault,
    Broadcast,
    Internal,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainError {
    #[error("invalid input: {0}")]
    Input(String),
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("connectivity error: {0}")]
    Connectivity(String),
    #[error("chain state error: {0}")]
    ChainState(String),
    #[error("authorization error: {0}")]
    Authorization(String),
    #[error("broadcast error: {0}")]
    Broadcast(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl ChainError {
    pub fn category(&self) -> ErrorCategory {
        match self {
            ChainError::Input(_) => ErrorCategory::Input,
            ChainError::Configuration(_) => ErrorCategory::Configuration,
            ChainError::Connectivity(_) => ErrorCategory::Connectivity,
            ChainError::ChainState(_) => ErrorCategory::ChainState,
            ChainError::Authorization(_) => ErrorCategory::Authorization,
            ChainError::Broadcast(_) => ErrorCategory::Broadcast,
            ChainError::Internal(_) => ErrorCategory::Internal,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WalletError {
    #[error("vault is missing")]
    VaultMissing,
    #[error("vault is corrupt")]
    VaultCorrupt,
    #[error("vault schema version {found} is not supported (need {required})")]
    VaultUnsupported { found: u32, required: u32 },
    #[error("vault integrity check failed")]
    VaultIntegrity,
    #[error("key derivation failed")]
    DerivationFailed,
    #[error("invalid mnemonic")]
    InvalidMnemonic,
    #[error("authentication failed: {0}")]
    Authentication(String),
    #[error("wallet locked")]
    Locked,
}