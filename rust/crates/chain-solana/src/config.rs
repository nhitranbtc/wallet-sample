//! Configuration for [`crate::SolanaAdapter`].
//!
//! [`SolanaConfig`] carries everything the Solana adapter needs to
//! speak devnet: the wallet-domain [`Network`] used in higher-level
//! descriptors, the [`rpc_client::EndpointConfig`] enforcing the
//! provider policy, and the JSON-RPC URL the adapter uses to fetch
//! recent blockhashes (the zero placeholder in
//! `PreparedPayload::Sol` for the architecture proof; a real
//! implementation would populate it from
//! `solana_rpc_client::rpc_client::RpcClient`).

use rpc_client::EndpointConfig;
use wallet_domain::account::Network;

/// Configuration passed to [`crate::SolanaAdapter::new`].
///
/// - `network` — the wallet-domain testnet/mainnet marker; the proof
///   only ever constructs adapters with `Network::Testnet`.
/// - `endpoint` — the RPC URL + provider policy validated by
///   [`rpc_client::EndpointConfig::validate`] before the adapter is
///   built (HTTPS-only, testnet-only, host allow-list).
/// - `rpc_url` — the JSON-RPC endpoint used to fetch the recent
///   blockhash for `PreparedPayload::Sol`. Kept distinct from
///   `endpoint.url` so the production allow-list and the
///   blockhash-fetch URL can diverge once the architecture proof
///   hands off.
#[derive(Debug, Clone)]
pub struct SolanaConfig {
    pub network: Network,
    pub endpoint: EndpointConfig,
    pub rpc_url: String,
}