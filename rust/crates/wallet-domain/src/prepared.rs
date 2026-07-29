use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PreparedPayload {
    Eth,
    Btc { change_index: u32 },
    Sol { blockhash: [u8; 32] },
    Tron { ref_block_bytes: [u8; 2], ref_block_hash: [u8; 8] },
}
