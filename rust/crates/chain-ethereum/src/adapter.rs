use std::str::FromStr;

use alloy::primitives::Address;
use async_trait::async_trait;
use chain_core::ChainAdapter;
use chrono::{Duration, Utc};
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

use crate::config::EthereumConfig;

/// Ethereum (Sepolia) implementation of [`ChainAdapter`].
#[derive(Debug)]
pub struct EthereumAdapter {
    config: EthereumConfig,
}

impl EthereumAdapter {
    /// Construct an adapter, rejecting any endpoint that fails policy
    /// validation (mainnet, non-HTTPS, or host outside the allow-list).
    ///
    /// Returns `Err(ChainError::Configuration)` instead of panicking so
    /// the failure is reportable across the FFI boundary.
    pub fn new(config: EthereumConfig) -> Result<Self, ChainError> {
        config.endpoint.validate()?;
        Ok(Self { config })
    }

    /// The EIP-155 chain id this adapter builds transactions for.
    pub fn chain_id(&self) -> u64 {
        self.config.chain_id
    }
}

#[async_trait]
impl ChainAdapter for EthereumAdapter {
    fn descriptor(&self) -> ChainDescriptor {
        ChainDescriptor {
            chain: ChainId::Ethereum,
            symbol: "ETH".into(),
            default_decimals: 18,
        }
    }

    async fn synchronize(&self, account: &AccountRef) -> Result<AccountSnapshot, ChainError> {
        // Bridge impl (follow-up): alloy provider `eth_getBalance` over the
        // derived address. Until the transport lands the snapshot is marked
        // `Stale` so callers never treat the zero balance as authoritative.
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

        // `Address::from_str` accepts both lowercase and checksummed hex;
        // it does not require EIP-55 casing, so a valid lowercase address
        // is not rejected as garbage.
        let _address = Address::from_str(&request.destination.0)
            .map_err(|_| ChainError::Input("destination is not a valid EVM address".into()))?;

        // Bridge impl (follow-up): alloy provider `eth_gasPrice` +
        // `eth_estimateGas` fill the network fee.
        let fee = FeeEstimate::from(ResourceSummary::NativeGas {
            network_fee: Amount(0),
            total_debit: request.amount,
        });

        Ok(PreparedTransfer {
            preparation_id: Uuid::new_v4().to_string(),
            source: request.source,
            destination: request.destination,
            amount: request.amount,
            fee,
            network: self.config.network,
            expires_at: Utc::now() + Duration::minutes(2),
            status: SnapshotStatus::Fresh,
            payload: PreparedPayload::Eth,
        })
    }

    async fn broadcast(&self, _signed: SignedEnvelope) -> Result<BroadcastReceipt, ChainError> {
        // Bridge impl (follow-up): alloy provider `eth_sendRawTransaction`.
        Ok(BroadcastReceipt {
            transaction_id: TransactionId("0x0".into()),
            submitted_at: Utc::now(),
        })
    }

    async fn transaction_status(
        &self,
        _id: &TransactionId,
    ) -> Result<TransactionStatus, ChainError> {
        // Bridge impl (follow-up): alloy provider
        // `eth_getTransactionReceipt` maps to Confirmed / Failed.
        Ok(TransactionStatus::Pending)
    }
}
