use crate::amount::Amount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceSummary {
    NativeGas { network_fee: Amount, total_debit: Amount },
    SatsVByte { fee_rate: u32, total_debit: Amount },
    LamportsAndComputeUnits { lamports: Amount, compute_units: u32, total_debit: Amount },
    TronBandwidthAndEnergy { bandwidth: u32, energy: u32, total_debit: Amount },
}
