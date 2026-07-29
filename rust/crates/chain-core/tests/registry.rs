use std::sync::Arc;

use async_trait::async_trait;
use chain_core::{ChainAdapter, ChainRegistry};
use wallet_domain::{
    account::{AccountRef, ChainId},
    broadcast::{BroadcastReceipt, SignedEnvelope, TransactionId, TransactionStatus},
    descriptor::ChainDescriptor,
    error::ChainError,
    snapshot::AccountSnapshot,
    transfer::{PreparedTransfer, TransferRequest},
};

/// Minimal `ChainAdapter` whose body methods always succeed with a
/// synthetic value. We only exercise the registry contract here; the
/// adapter itself is smoke-tested by the per-chain crates.
struct NoopAdapter {
    desc: ChainDescriptor,
}

#[async_trait]
impl ChainAdapter for NoopAdapter {
    fn descriptor(&self) -> ChainDescriptor {
        self.desc.clone()
    }

    async fn synchronize(
        &self,
        _account: &AccountRef,
    ) -> Result<AccountSnapshot, ChainError> {
        Err(ChainError::Internal("noop adapter".into()))
    }

    async fn prepare_transfer(
        &self,
        _request: TransferRequest,
    ) -> Result<PreparedTransfer, ChainError> {
        Err(ChainError::Internal("noop adapter".into()))
    }

    async fn broadcast(
        &self,
        _signed: SignedEnvelope,
    ) -> Result<BroadcastReceipt, ChainError> {
        Err(ChainError::Internal("noop adapter".into()))
    }

    async fn transaction_status(
        &self,
        _transaction_id: &TransactionId,
    ) -> Result<TransactionStatus, ChainError> {
        Err(ChainError::Internal("noop adapter".into()))
    }
}

fn eth_descriptor() -> ChainDescriptor {
    ChainDescriptor {
        chain: ChainId::Ethereum,
        symbol: "ETH".to_string(),
        default_decimals: 18,
    }
}

fn btc_descriptor() -> ChainDescriptor {
    ChainDescriptor {
        chain: ChainId::Bitcoin,
        symbol: "BTC".to_string(),
        default_decimals: 8,
    }
}

#[tokio::test]
async fn registry_round_trips_descriptor() {
    let mut registry = ChainRegistry::new();
    let eth: Arc<dyn ChainAdapter> = Arc::new(NoopAdapter {
        desc: eth_descriptor(),
    });
    registry.register(ChainId::Ethereum, eth.clone());

    let looked_up = registry
        .get(ChainId::Ethereum)
        .expect("ethereum adapter should be present");
    assert_eq!(looked_up.descriptor().chain, ChainId::Ethereum);
    assert_eq!(looked_up.descriptor().symbol, "ETH");
    assert_eq!(looked_up.descriptor().default_decimals, 18);

    assert!(registry.get(ChainId::Bitcoin).is_none());
    assert_eq!(registry.list().len(), 1);
}

#[tokio::test]
async fn registry_list_returns_all_registered_chains() {
    let mut registry = ChainRegistry::new();
    registry.register(
        ChainId::Ethereum,
        Arc::new(NoopAdapter {
            desc: eth_descriptor(),
        }),
    );
    registry.register(
        ChainId::Bitcoin,
        Arc::new(NoopAdapter {
            desc: btc_descriptor(),
        }),
    );

    let listed = registry.list();
    assert_eq!(listed.len(), 2);

    let chains: Vec<ChainId> = listed.iter().map(|(c, _)| *c).collect();
    assert!(chains.contains(&ChainId::Ethereum));
    assert!(chains.contains(&ChainId::Bitcoin));
}

#[tokio::test]
async fn registry_register_replaces_existing_adapter() {
    let mut registry = ChainRegistry::new();
    registry.register(
        ChainId::Ethereum,
        Arc::new(NoopAdapter {
            desc: eth_descriptor(),
        }),
    );

    let replacement: Arc<dyn ChainAdapter> = Arc::new(NoopAdapter {
        desc: ChainDescriptor {
            chain: ChainId::Ethereum,
            symbol: "WETH".to_string(),
            default_decimals: 18,
        },
    });
    registry.register(ChainId::Ethereum, replacement.clone());

    assert_eq!(registry.list().len(), 1);
    let looked_up = registry.get(ChainId::Ethereum).unwrap();
    assert_eq!(looked_up.descriptor().symbol, "WETH");
}

#[test]
fn empty_registry_lists_nothing() {
    let registry = ChainRegistry::new();
    assert!(registry.list().is_empty());
    assert!(registry.get(ChainId::Ethereum).is_none());
}