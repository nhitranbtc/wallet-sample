//! Solana (devnet) implementation of [`chain_core::ChainAdapter`].
//!
//! Construction is fallible: the adapter refuses to exist unless its
//! [`SolanaConfig`] carries an endpoint that passes
//! `rpc_client::EndpointConfig::validate` (HTTPS-only, testnet-only,
//! host allow-list). The adapter never panics.
//!
//! The architecture proof declares `solana-signer` and
//! `solana-rpc-client` as dependencies but does **not** yet call
//! either: the ed25519 signature path and the recent-blockhash /
//! broadcast / status transport are deferred. Every call site that
//! will consume them carries a `FIXME:` below so the unused deps are
//! visible rather than silently claimed. See
//! `wallet-sample/docs/manual-test-release-2.md` ("Release 2
//! broadcast wiring (deferred)") for the user-facing consequence.
//!
//! Address validation is base58 decode + a 32-byte length check plus
//! an ed25519 identity / small-subgroup rejection — see
//! [`is_valid_solana_pubkey`]. `prepare_transfer` builds a
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
/// Solana pubkeys are 32-byte (compressed) Edwards points. We reject:
///   - Wrong length / non-base58: bs58 decode + length check
///   - Identity point (all-zero bytes): explicit byte check
///   - Small-subgroup points that are mathematically valid but
///     vulnerable to signature malleability: ed25519-dalek's
///     `is_weak()`
///
/// Real wallets and CLI tools perform equivalent checks.
///
/// # Why the decoder alone is not enough
///
/// `ed25519_dalek::VerifyingKey::from_bytes` rejects malformed
/// encodings but **accepts** low-order and identity points. The
/// System Program address `11111111111111111111111111111111`
/// decodes to 32 zero bytes, which is a decodable order-4 point, so
/// `from_bytes` returns `Ok`. Accepting it would let a caller
/// prepare a transfer that burns funds, and pairing a crafted
/// small-order pubkey with a known weak secret is the classic
/// signing-substitute attack. Hence the extra `is_weak()` and
/// all-zero guards below.
fn is_valid_solana_pubkey(s: &str) -> bool {
    let raw = match bs58::decode(s).into_vec() {
        Ok(b) => b,
        Err(_) => return false,
    };
    if raw.len() != 32 {
        return false;
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&raw);
    let vk = match VerifyingKey::from_bytes(&bytes) {
        Ok(vk) => vk,
        Err(_) => return false,
    };
    // `is_weak()` covers the small-subgroup points; the explicit
    // all-zero comparison pins the identity-point reject so the
    // guarantee survives any future change in dalek's definition of
    // "weak".
    !vk.is_weak() && bytes != [0u8; 32]
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
            // FIXME(Release 2 broadcast wiring): fetch the real hash
            // with `solana_rpc_client::RpcClient::get_latest_blockhash`
            // and embed it here. Until then this zero placeholder
            // makes the prepared payload unsignable/unbroadcastable
            // on devnet, and `solana-rpc-client` stays an unused
            // dependency in Cargo.toml.
            payload: PreparedPayload::Sol {
                blockhash: [0u8; 32],
            },
        })
    }

    async fn broadcast(
        &self,
        _signed: SignedEnvelope,
    ) -> Result<BroadcastReceipt, ChainError> {
        // FIXME(Release 2 broadcast wiring): sign via
        // `solana_signer::Signer` (see `signer::build_signer`) and
        // submit with `solana_rpc_client::RpcClient::send_transaction`.
        // The empty receipt below is a placeholder — it reports no
        // signature, so callers cannot poll it. `ffi-bridge`'s
        // `authenticate_sign_and_broadcast` therefore refuses the
        // Solana arm outright instead of reaching this code.
        Ok(BroadcastReceipt {
            transaction_id: TransactionId(String::new()),
            submitted_at: Utc::now(),
        })
    }

    async fn transaction_status(
        &self,
        _id: &TransactionId,
    ) -> Result<TransactionStatus, ChainError> {
        // FIXME(Release 2 broadcast wiring): map
        // `solana_rpc_client::RpcClient::get_signature_statuses` onto
        // Confirmed / Failed. Hard-coded `Pending` means a polling
        // caller never terminates, which is why the manual-test
        // checklist marks the poll step blocked.
        Ok(TransactionStatus::Pending)
    }
}