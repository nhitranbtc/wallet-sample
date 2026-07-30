//! [`WalletSummary`] — returned by `create_wallet` and
//! `restore_wallet_via_native_surface`. Carries a wallet id and the
//! account-zero `(ChainId, ChainDescriptor, address)` tuple for every
//! chain the wallet has been enrolled into.

use serde::{Deserialize, Serialize};
use wallet_domain::account::ChainId;
use wallet_domain::descriptor::ChainDescriptor;

/// Result of a wallet create / restore operation.
///
/// `wallet_id` is the opaque UUID v4 handed back to Dart. `accounts`
/// is the ordered list of `(chain, descriptor, address)` tuples for
/// account zero, one entry per chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletSummary {
    pub wallet_id: String,
    pub accounts: Vec<(ChainId, ChainDescriptor, String)>,
}
