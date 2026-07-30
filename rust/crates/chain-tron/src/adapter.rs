//! Tron (Shasta) implementation of [`chain_core::ChainAdapter`].
//!
//! Construction is fallible: the adapter refuses to exist unless its
//! [`TronConfig`] carries an endpoint that passes
//! `rpc_client::EndpointConfig::validate` (HTTPS-only, testnet-only,
//! host allow-list). The adapter never panics.
//!
//! The architecture proof stores the protocol chain id and accepts
//! `PreparedPayload::Tron { ref_block_bytes, ref_block_hash }` with the
//! zero-byte placeholder for the block reference (a real
//! implementation fetches it via `trongrid::wallet/getnowblock`); no
//! real RPC request is made.
//!
//! Address validation uses the strict Tron base58check format: a 34
//! character base58 string starting with `T` that decodes to 25 bytes
//! whose trailing 4 bytes are the double-SHA256 checksum of the
//! preceding 21 bytes (`0x41` mainnet prefix byte + 20-byte address).
//! `prepare_transfer` builds a `ResourceSummary::TronBandwidthAndEnergy`
//! with the proof's constant 600 / 65_000 TRX-bandwidth / energy
//! budget and a `PreparedPayload::Tron { ref_block_bytes [0, 0],
//! ref_block_hash [0; 8] }`.

use async_trait::async_trait;
use chain_core::ChainAdapter;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
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

use crate::config::TronConfig;

/// Tron (Shasta) implementation of [`ChainAdapter`].
#[derive(Debug)]
pub struct TronAdapter {
    config: TronConfig,
}

impl TronAdapter {
    /// Construct an adapter, rejecting any endpoint that fails policy
    /// validation (mainnet, non-HTTPS, or host outside the
    /// allow-list). Returns `Err(ChainError::Configuration)` instead
    /// of panicking so the failure is reportable across the FFI
    /// boundary.
    pub fn new(config: TronConfig) -> Result<Self, ChainError> {
        config.endpoint.validate()?;
        Ok(Self { config })
    }

    /// The protocol chain id this adapter signs transactions for.
    pub fn chain_id(&self) -> u64 {
        self.config.chain_id
    }
}

/// Validate a base58-encoded Tron mainnet address.
///
/// A valid Tron mainnet address is a 34-character base58 string that
/// decodes to 25 bytes laid out as:
/// - 1 byte: `0x41` (Tron mainnet prefix).
/// - 20 bytes: the Keccak-256-derived address (last 20 bytes of the
///   keccak256 of the uncompressed secp256k1 pubkey).
/// - 4 bytes: the double-SHA256 (SHA-256 ∘ SHA-256) checksum of the
///   preceding 21 bytes, truncated to its first 4 bytes.
///
/// The validator is symmetric with the Bitcoin bech32 validator:
/// length + charset + checksum. A malformed base58 string, a
/// non-`T` prefix, a non-`0x41` body prefix, a wrong decoded length,
/// or a mismatched checksum all return `false`. The empty string
/// returns `false` so callers cannot silently hand an empty
/// destination over to `prepare_transfer`.
///
/// # Task 17 contract
///
/// The double-SHA256 checksum is the standard base58check pattern
/// (Bitcoin / Cosmos / Tron all use SHA-256 ∘ SHA-256 over the body
/// and take the leading 4 bytes as the checksum). `sha2::Sha256` is a
/// well-defined constant-time hash; an incorrect body still produces
/// a deterministic 32-byte digest, so the trailing 4-byte comparison
/// fails fast for any body whose checksum does not match.
fn is_valid_tron_address(s: &str) -> bool {
    if s.is_empty() || s.len() != 34 || !s.starts_with('T') {
        return false;
    }
    let raw = match bs58::decode(s).into_vec() {
        Ok(b) => b,
        Err(_) => return false,
    };
    if raw.len() != 25 {
        return false;
    }
    // Tron mainnet prefix is `0x41`. Shasta addresses carry the same
    // byte because Tron does not have a separate testnet prefix on the
    // address layer — the testnet is signalled by the JSON-RPC
    // endpoint, not by the address.
    if raw[0] != 0x41 {
        return false;
    }
    let checksum = &raw[21..25];
    let body = &raw[..21];
    let mut h = Sha256::new();
    h.update(body);
    let first = h.finalize();
    let mut h = Sha256::new();
    h.update(first);
    let second = h.finalize();
    &second[..4] == checksum
}

#[async_trait]
impl ChainAdapter for TronAdapter {
    fn descriptor(&self) -> ChainDescriptor {
        ChainDescriptor {
            chain: ChainId::Tron,
            symbol: "TRX".into(),
            default_decimals: 6,
        }
    }

    async fn synchronize(
        &self,
        account: &AccountRef,
    ) -> Result<AccountSnapshot, ChainError> {
        // Bridge impl (follow-up): `api.shasta.trongrid.io`
        // `wallet/getaccount` over the derived Tron address. Until
        // the transport lands the snapshot is marked `Stale` so
        // callers never treat the zero balance as authoritative.
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
        if !is_valid_tron_address(&request.destination.0) {
            return Err(ChainError::Input(
                "destination is not a valid Tron address".into(),
            ));
        }

        // Fee estimate sits at the proof's constant 600 TRX-bandwidth
        // and 65_000 energy budget; total_debit is the full request
        // amount so callers can see exactly what they are paying
        // on-chain at the prepare stage, before any real
        // `wallet/estimateenergy` replacement.
        let fee = FeeEstimate::from(ResourceSummary::TronBandwidthAndEnergy {
            bandwidth: 600,
            energy: 65_000,
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
            // `api.shasta.trongrid.io/wallet/getnowblock` and embed
            // the 2-byte `ref_block_bytes` + 8-byte `ref_block_hash`
            // here. The architecture proof accepts the zero
            // placeholder so the contract tests do not need network
            // access.
            payload: PreparedPayload::Tron {
                ref_block_bytes: [0u8, 0u8],
                ref_block_hash: [0u8; 8],
            },
        })
    }

    async fn broadcast(
        &self,
        _signed: SignedEnvelope,
    ) -> Result<BroadcastReceipt, ChainError> {
        // Bridge impl (follow-up): `api.shasta.trongrid.io`
        // `wallet/broadcasttransaction` over the signed envelope.
        Ok(BroadcastReceipt {
            transaction_id: TransactionId(String::new()),
            submitted_at: Utc::now(),
        })
    }

    async fn transaction_status(
        &self,
        _id: &TransactionId,
    ) -> Result<TransactionStatus, ChainError> {
        // Bridge impl (follow-up): `api.shasta.trongrid.io`
        // `wallet/gettransactioninfobyid` maps to Confirmed / Failed.
        Ok(TransactionStatus::Pending)
    }
}
