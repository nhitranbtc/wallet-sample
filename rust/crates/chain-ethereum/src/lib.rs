//! Ethereum chain adapter for the multi-chain wallet architecture proof.
//!
//! [`EthereumAdapter`] implements `chain_core::ChainAdapter` on top of
//! [`alloy`]. Construction is fallible: the adapter refuses to exist
//! unless its [`EthereumConfig`] carries an endpoint that passes
//! `rpc_client::EndpointConfig::validate` (HTTPS-only, testnet-only,
//! host allow-list). The adapter never panics.

pub mod adapter;
pub mod config;
pub mod signer;

pub use adapter::EthereumAdapter;
pub use config::EthereumConfig;
