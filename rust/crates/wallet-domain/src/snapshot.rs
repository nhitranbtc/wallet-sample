use crate::account::{AccountRef, AddressDisplay};
use crate::amount::Amount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotStatus {
    Fresh,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountSnapshot {
    pub account: AccountRef,
    pub address: AddressDisplay,
    pub balance: Amount,
    pub fetched_at: DateTime<Utc>,
    pub status: SnapshotStatus,
}