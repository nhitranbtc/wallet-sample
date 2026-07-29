use async_trait::async_trait;
use wallet_domain::{
    account::AccountRef,
    broadcast::{BroadcastReceipt, SignedEnvelope, TransactionId, TransactionStatus},
    descriptor::ChainDescriptor,
    error::ChainError,
    snapshot::AccountSnapshot,
    transfer::{PreparedTransfer, TransferRequest},
};

/// Object-safe adapter contract for any chain implementation.
///
/// An adapter exposes the five operations the wallet-orchestration layer
/// needs in order to keep an account fresh and move value out of it.
/// Implementors must be `Send + Sync` so they can be stored behind an
/// `Arc<dyn ChainAdapter>` and shared across async tasks.
///
/// Capabilities (native transfer, resource fees, blockhash freshness)
/// are exposed per-adapter on the `ChainDescriptor` rather than via a
/// runtime bitfield, so the surface area here stays small and stable.
#[async_trait]
pub trait ChainAdapter: Send + Sync {
    /// Return a descriptor describing the chain this adapter speaks for.
    fn descriptor(&self) -> ChainDescriptor;

    /// Refresh on-chain state for `account` and return a snapshot.
    async fn synchronize(
        &self,
        account: &AccountRef,
    ) -> Result<AccountSnapshot, ChainError>;

    /// Build a `PreparedTransfer` for `request` including a fee estimate
    /// and the chain-specific payload needed to sign.
    async fn prepare_transfer(
        &self,
        request: TransferRequest,
    ) -> Result<PreparedTransfer, ChainError>;

    /// Submit a previously-signed envelope to the network.
    async fn broadcast(
        &self,
        signed: SignedEnvelope,
    ) -> Result<BroadcastReceipt, ChainError>;

    /// Look up the current status of a transaction.
    async fn transaction_status(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionStatus, ChainError>;
}