use super::{transaction_input::TransactionInput, transaction_output::TransactionOutput};

#[derive(Clone)]
pub struct Transaction {
    pub version: u32,
    pub reward: u64, // Only used in the very first transaction of each block, should include the block value + the sum of all transaction fees in the block
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub locktime: u32,
}

impl Transaction {
    pub fn hash(&self) -> u32 {
        // Dummy hash function. Ww return the `locktime` field, which in reality is used for something else entirely.
        self.locktime
    }
}
