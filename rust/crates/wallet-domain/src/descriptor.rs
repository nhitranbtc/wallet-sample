use crate::account::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainDescriptor {
    pub chain: ChainId,
    pub symbol: String,
    pub default_decimals: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainCapabilities {
    pub supports_native_transfer: bool,
    pub supports_resource_fees: bool,
    pub supports_blockhash_freshness: bool,
}