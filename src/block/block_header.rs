use crate::constants::VERSION;

pub struct BlockHeader {
    pub version: u32,
    pub previous_block_hash: u32, // 4 bytes instead of 256 for convenience
    pub merkle_root: u32,
    pub timestamp: u32,
    pub difficulty_target: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn genesis() -> Self {
        Self {
            version: VERSION,
            previous_block_hash: 0x0,
            merkle_root: 0x0,
            timestamp: 0,
            difficulty_target: 0,
            nonce: 0,
        }
    }
}
