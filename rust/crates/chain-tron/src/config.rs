//! Configuration for [`crate::TronAdapter`].
//!
//! [`TronConfig`] carries everything the Tron adapter needs to speak
//! Shasta: the wallet-domain [`Network`] used in higher-level
//! descriptors, the [`rpc_client::EndpointConfig`] enforcing the
//! provider policy, and the EIP-155 / Tron protocol chain id used in
//! transaction signing.

use rpc_client::EndpointConfig;
use wallet_domain::account::Network;

/// Configuration passed to [`crate::TronAdapter::new`].
///
/// - `network` — the wallet-domain testnet/mainnet marker; the proof
///   only ever constructs adapters with `Network::Testnet`.
/// - `endpoint` — the RPC URL + provider policy validated by
///   [`rpc_client::EndpointConfig::validate`] before the adapter is
///   built (HTTPS-only, testnet-only, host allow-list).
/// - `chain_id` — the protocol chain id used when signing the
///   Tron transaction envelope. Shasta is `2494104990` (also written
///   as the hex literal `0x94a9059e56`); mainnet is `728126428`.
///   The proof keeps it as a `u64` so the value flows into the
///   signing machinery without an extra cast.
#[derive(Debug, Clone)]
pub struct TronConfig {
    pub network: Network,
    pub endpoint: EndpointConfig,
    pub chain_id: u64,
}
