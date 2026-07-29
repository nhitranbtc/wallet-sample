use crate::account::{AccountRef, AddressDisplay, Network};
use crate::amount::Amount;
use crate::error::ChainError;
use crate::snapshot::SnapshotStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRequest {
    pub source: AccountRef,
    pub destination: AddressDisplay,
    pub amount: Amount,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub network_fee: Amount,
    pub total_debit: Amount,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedTransfer {
    pub preparation_id: String,
    pub source: AccountRef,
    pub destination: AddressDisplay,
    pub amount: Amount,
    pub fee: FeeEstimate,
    pub network: Network,
    pub expires_at: DateTime<Utc>,
    pub status: SnapshotStatus,
}

impl PreparedTransfer {
    pub fn validate_fresh(&self, source: &AccountRef, network: Network) -> Result<(), ChainError> {
        if self.source != *source {
            return Err(ChainError::ChainState("account changed".into()));
        }
        if self.network != network {
            return Err(ChainError::ChainState("network changed".into()));
        }
        if self.expires_at <= Utc::now() {
            return Err(ChainError::ChainState("preparation expired".into()));
        }
        Ok(())
    }
}