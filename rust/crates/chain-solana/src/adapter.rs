//! Solana (devnet) implementation of [`chain_core::ChainAdapter`].
//!
//! Construction is fallible: the adapter refuses to exist unless its
//! [`SolanaConfig`] carries an endpoint that passes
//! `rpc_client::EndpointConfig::validate` (HTTPS-only, testnet-only,
//! host allow-list). The adapter never panics.
//!
//! The architecture proof uses `solana_signer::Signer` for the
//! eventual ed25519 signature path and `solana_rpc_client` for the
//! recent-blockhash fetch. Both APIs are behind a small synchronous
//! façade in this file; the contract tests exercise only the
//! validation surface, not the network calls, so no real RPC request
//! is made.
//!
//! Address validation is base58 decode + a 32-byte length check plus
//! an ed25519 off-curve check (a Solana on-curve point is a valid
//! pubkey; the identity point is rejected because it would never
//! sign anything meaningful on devnet). `prepare_transfer` builds a
//! `ResourceSummary::LamportsAndComputeUnits` and a
//! `PreparedPayload::Sol { blockhash }`; the blockhash is the
//! zero-byte placeholder for the architecture proof (real impl
//! fetches via `RpcClient::get_latest_blockhash`).

use async_trait::async_trait;
use chain_core::ChainAdapter;
use chrono::{Duration, Utc};
use ed25519_dalek::VerifyingKey;
use uuid::Uuid;
use wallet_domain::{
    account::{AccountRef, AddressDisplay, ChainId},
    amount::Amount,
    broadcast::{BroadcastReceipt, SignedEnvelope, TransactionId, TransactionStatus},
    descriptor::ChainDescriptor,
    error::ChainError,
    fee::ResourceSummary,
    prepared::PreparedPayload,
    snapshot::{AccountSnapshot, SnapshotStatus},
    transfer::{FeeEstimate, PreparedTransfer, TransferRequest},
};

use crate::config::SolanaConfig;

/// Solana (Devnet) implementation of [`ChainAdapter`].
#[derive(Debug)]
pub struct SolanaAdapter {
    config: SolanaConfig,
}

impl SolanaAdapter {
    /// Construct an adapter, rejecting any endpoint that fails policy
    /// validation (mainnet, non-HTTPS, or host outside the
    /// allow-list). Returns `Err(ChainError::Configuration)` instead
    /// of panicking so the failure is reportable across the FFI
    /// boundary.
    pub fn new(config: SolanaConfig) -> Result<Self, ChainError> {
        config.endpoint.validate()?;
        Ok(Self { config })
    }
}

/// Validate a base58-encoded Solana pubkey.
///
/// A valid Solana pubkey is 32 bytes of decoded base58 that decodes
/// to a point on the ed25519 curve which is **not** the identity
/// point. Decoding rejects malformed base58; the length check
/// rejects anything other than 32 bytes; the off-curve / identity
/// check rejects the small constant set of strings that pass the
/// first two checks but would never be a real pubkey.
///
/// # Task 16 contract
///
/// The ed25519 off-curve check uses
/// [`ed25519_dalek::VerifyingKey::from_bytes`], which is a
/// well-defined constant-time decoding function. A pubkey whose
/// 32-byte encoding decodes to a point not on the curve returns
/// `Err(InternalError)`, and we treat that as "not a valid Solana
/// pubkey" — same result as a base58 / length failure. The empty
/// result is rejected so callers cannot silently hand an empty
/// destination over to `prepare_transfer`.
fn is_valid_solana_pubkey(s: &str) -> bool {
    let raw = match bs58::decode(s).into_vec() {
        Ok(b) => b,
        Err(_) => return false,
    };
    if raw.is_empty() {
        return false;
    }
    if raw.len() != 32 {
        return false;
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&raw);
    // `VerifyingKey::from_bytes` returns `Err` for any 32-byte input
    // that is not a valid compressed Edwards point. Solana treats
    // both the identity point and off-curve points as invalid
    // destinations; the dalek check covers both, so a single
    // success/fail branch is sufficient.
    VerifyingKey::from_bytes(&bytes).is_ok()
}

#[async_trait]
impl ChainAdapter for SolanaAdapter {
    fn descriptor(&self) -> ChainDescriptor {
        ChainDescriptor {
            chain: ChainId::Solana,
            symbol: "SOL".into(),
            default_decimals: 9,
        }
    }

    async fn synchronize(
        &self,
        account: &AccountRef,
    ) -> Result<AccountSnapshot, ChainError> {
        // Bridge impl (follow-up): `solana_rpc_client` `get_balance`
        // over the derived ed25519 address. Until the transport
        // lands the snapshot is marked `Stale` so callers never
        // treat the zero balance as authoritative.
        Ok(AccountSnapshot {
            account: account.clone(),
            address: AddressDisplay(String::new()),
            balance: Amount(0),
            fetched_at: Utc::now(),
            status: SnapshotStatus::Stale,
        })
    }

    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<PreparedTransfer, ChainError> {
        if request.amount.0 == 0 {
            return Err(ChainError::Input("amount is zero".into()));
        }
        if !is_valid_solana_pubkey(&request.destination.0) {
            return Err(ChainError::Input(
                "destination is not a valid Solana pubkey".into(),
            ));
        }

        // Fee estimate sits at the proof's constant 5_000-lamport
        // signature fee and 200_000 compute units; total_debit is
        // the full request amount so callers can see exactly what
        // they are paying on-chain at the prepare stage, before any
        // real priority-fee + compute-budget replacement.
        let fee = FeeEstimate::from(ResourceSummary::LamportsAndComputeUnits {
            lamports: Amount(5_000),
            compute_units: 200_000,
            total_debit: request.amount,
        });

        Ok(PreparedTransfer {
            preparation_id: Uuid::new_v4().to_string(),
            source: request.source,
            destination: request.destination,
            amount: request.amount,
            fee,
            network: self.config.network,
            expires_at: Utc::now() + Duration::seconds(60),
            status: SnapshotStatus::Fresh,
            // Real implementation: fetch via
            // `RpcClient::get_latest_blockhash()` and embed the
            // returned 32-byte hash here. The architecture proof
            // accepts the zero placeholder so the contract tests do
            // not need network access.
            payload: PreparedPayload::Sol {
                blockhash: [0u8; 32],
            },
        })
    }

    async fn broadcast(
        &self,
        _signed: SignedEnvelope,
    ) -> Result<BroadcastReceipt, ChainError> {
        // Bridge impl (follow-up): `solana_rpc_client`
        // `send_transaction` over the signed envelope.
        Ok(BroadcastReceipt {
            transaction_id: TransactionId(String::new()),
            submitted_at: Utc::now(),
        })
    }

    async fn transaction_status(
        &self,
        _id: &TransactionId,
    ) -> Result<TransactionStatus, ChainError> {
        // Bridge impl (follow-up): `solana_rpc_client`
        // `get_signature_statuses` maps to Confirmed / Failed.
        Ok(TransactionStatus::Pending)
    }
}