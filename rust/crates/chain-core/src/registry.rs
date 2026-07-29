use std::collections::HashMap;
use std::sync::Arc;

use wallet_domain::account::ChainId;

use crate::adapter::ChainAdapter;

/// Registry of installed chain adapters keyed by [`ChainId`].
///
/// The FFI bridge reads [`ChainRegistry::list`] to populate
/// `list_chains`; the orchestration layer reads [`ChainRegistry::get`]
/// to dispatch a request to the right adapter.
pub struct ChainRegistry {
    map: HashMap<ChainId, Arc<dyn ChainAdapter>>,
}

impl ChainRegistry {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Install (or replace) the adapter for `chain`.
    pub fn register(&mut self, chain: ChainId, adapter: Arc<dyn ChainAdapter>) {
        self.map.insert(chain, adapter);
    }

    /// Look up the adapter for `chain`.
    pub fn get(&self, chain: ChainId) -> Option<Arc<dyn ChainAdapter>> {
        self.map.get(&chain).cloned()
    }

    /// Snapshot every registered `(chain, adapter)` pair.
    ///
    /// The order is not stable; callers that need a stable order should
    /// sort the returned vector.
    pub fn list(&self) -> Vec<(ChainId, Arc<dyn ChainAdapter>)> {
        self.map
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
}

impl Default for ChainRegistry {
    fn default() -> Self {
        Self::new()
    }
}