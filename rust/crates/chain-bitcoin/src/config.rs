//! Configuration for [`crate::BitcoinAdapter`].
//!
//! [`BitcoinConfig`] carries everything the Bitcoin adapter needs to
//! speak testnet: the wallet-domain [`Network`] used in higher-level
//! descriptors, the [`rpc_client::EndpointConfig`] enforcing the
//! provider policy, the [`BdkNet`] consumed by `bdk_wallet`, and the
//! on-disk path of the AES-256-GCM encrypted BDK change-set store.

use bdk_wallet::bitcoin::Network as BdkNet;
use rpc_client::EndpointConfig;
use wallet_domain::account::Network;

/// Configuration passed to [`crate::BitcoinAdapter::new`].
///
/// - `network` — the wallet-domain testnet/mainnet marker; the proof
///   only ever constructs adapters with `Network::Testnet`.
/// - `endpoint` — the RPC URL + provider policy validated by
///   [`rpc_client::EndpointConfig::validate`] before the adapter is
///   built.
/// - `bdk_network` — the `bdk_wallet::bitcoin::Network` driving script
///   prefix selection (testnet vs mainnet bech32 HRP).
/// - `encrypted_db_path` — the on-disk path of the SQLite file that
///   [`crate::persistence::EncryptedBdkStore`] uses; the file holds
///   the AES-256-GCM ciphertext of the BDK change set, never the
///   plaintext index counter.
#[derive(Debug, Clone)]
pub struct BitcoinConfig {
    pub network: Network,
    pub endpoint: EndpointConfig,
    pub bdk_network: BdkNet,
    pub encrypted_db_path: String,
}
