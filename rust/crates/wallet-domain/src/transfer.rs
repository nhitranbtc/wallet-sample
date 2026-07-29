use crate::account::{AccountRef, AddressDisplay, Network};
use crate::amount::Amount;
use crate::error::ChainError;
use crate::fee::ResourceSummary;
use crate::prepared::PreparedPayload;
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
    pub resource: ResourceSummary,
}

impl From<ResourceSummary> for FeeEstimate {
    fn from(resource: ResourceSummary) -> Self {
        Self { resource }
    }
}

impl FeeEstimate {
    pub fn resource(resource: ResourceSummary) -> Self {
        Self::from(resource)
    }

    pub fn native_gas(network_fee: Amount, total_debit: Amount) -> Self {
        Self::from(ResourceSummary::NativeGas { network_fee, total_debit })
    }
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
    pub payload: PreparedPayload,
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
