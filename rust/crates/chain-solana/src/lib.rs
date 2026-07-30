//! Solana chain adapter for the multi-chain wallet architecture proof.
//!
//! [`SolanaAdapter`] implements `chain_core::ChainAdapter` on top of
//! `solana_signer::Signer` and `solana_rpc_client`. Construction is
//! fallible: the adapter refuses to exist unless its [`SolanaConfig`]
//! carries an endpoint that passes
//! `rpc_client::EndpointConfig::validate` (HTTPS-only, testnet-only,
//! host allow-list). The adapter never panics.
//!
//! Address validation uses base58 decode + a 32-byte length check
//! plus an ed25519 off-curve check via
//! `ed25519_dalek::VerifyingKey::from_bytes`. The blockhash carried
//! in `PreparedPayload::Sol` is the zero-byte placeholder for the
//! architecture proof; a real implementation fetches it via
//! `solana_rpc_client::RpcClient::get_latest_blockhash`.

pub mod adapter;
pub mod config;
pub mod signer;

pub use adapter::SolanaAdapter;
pub use config::SolanaConfig;