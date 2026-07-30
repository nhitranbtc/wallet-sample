//! Tron chain adapter for the multi-chain wallet architecture proof.
//!
//! [`TronAdapter`] implements `chain_core::ChainAdapter` on top of the
//! Tron's BIP-44 secp256k1 derivation path (`m/44'/195'/0'/0/0`) and the
//! `api.shasta.trongrid.io` JSON-RPC endpoint. Construction is
//! fallible: the adapter refuses to exist unless its [`TronConfig`]
//! carries an endpoint that passes
//! `rpc_client::EndpointConfig::validate` (HTTPS-only, testnet-only,
//! host allow-list). The adapter never panics.
//!
//! Address validation uses the strict Tron base58check format: a 34
//! character base58 string starting with `T` that decodes to 25 bytes
//! whose trailing 4 bytes are the double-SHA256 checksum of the
//! preceding 21 bytes (the `0x41` mainnet prefix byte + 20-byte
//! address). The block reference carried in
//! `PreparedPayload::Tron { ref_block_bytes, ref_block_hash }` is the
//! zero-byte placeholder for the architecture proof; a real
//! implementation fetches it via
//! `trongrid::v1::wallet/getnowblock`.

pub mod adapter;
pub mod config;
pub mod signer;

pub use adapter::TronAdapter;
pub use config::TronConfig;
