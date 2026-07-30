//! [`WalletStatus`] — the only shape the FFI surface exposes for the
//! "is the wallet ready" question.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wallet_domain::descriptor::ChainDescriptor;

/// Snapshot of the wallet returned by `wallet_status`.
///
/// `initialized` and `locked` are derived from
/// [`wallet_orchestration::SessionState`] phase; `enabled_chains` is the
/// live set of chains with adapters installed in the
/// `chain_core::ChainRegistry`; `last_sync_at` is the timestamp of the
/// most recent `refresh_accounts` call (currently `None`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletStatus {
    pub initialized: bool,
    pub locked: bool,
    pub enabled_chains: Vec<ChainDescriptor>,
    pub last_sync_at: Option<DateTime<Utc>>,
}
