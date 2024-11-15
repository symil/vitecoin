use super::block_header::BlockHeader;
use crate::transaction::transaction::Transaction;

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn hash(&self) -> u32 {
        // Dummy hash function, which doesn't hash anything.
        // It just returns the noonce.

        self.header.nonce
    }
}
