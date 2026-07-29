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
/// The previous proof plan exposed a `capabilities()` method on `ChainAdapter`
/// that returned a `ChainCapabilities` struct. We omit it here — capabilities
/// have not yet been modeled in `wallet-domain` (see `ChainCapabilities`
/// in `wallet-domain::descriptor`). When modeling is added later, attach
/// fields directly to `ChainDescriptor` rather than a separate runtime
/// bitfield so the contract is a single source of truth.
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