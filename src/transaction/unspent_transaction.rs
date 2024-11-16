use super::transaction_output::TransactionOutput;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UnspentTransaction {
    pub hash: u32,
    pub unspent_outputs: HashMap<u32, TransactionOutput>,
}

impl UnspentTransaction {
    pub fn new(hash: u32) -> Self {
        Self {
            hash,
            unspent_outputs: HashMap::new(),
        }
    }
}
