//! Bitcoin chain adapter for the multi-chain wallet architecture proof.
//!
//! [`BitcoinAdapter`] implements `chain_core::ChainAdapter` on top of
//! `bdk_wallet` + `bdk_esplora`. Construction is fallible: the adapter
//! refuses to exist unless its [`BitcoinConfig`] carries an endpoint
//! that passes `rpc_client::EndpointConfig::validate` (HTTPS-only,
//! testnet-only, host allow-list). The adapter never panics.
//!
//! Address validation uses the BIP-173 bech32 polymod; the on-disk
//! BDK change set is sealed with the AES-256-GCM
//! `secure_storage::Vault` before it reaches SQLite.

pub mod adapter;
pub mod config;
pub mod persistence;
pub mod signer;

pub use adapter::BitcoinAdapter;
pub use config::BitcoinConfig;
pub use persistence::EncryptedBdkStore;
