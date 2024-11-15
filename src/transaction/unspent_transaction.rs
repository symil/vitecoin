use std::collections::HashMap;
use super::transaction_output::TransactionOutput;

#[derive(Default)]
pub struct UnspentTransaction {
    pub unspent_outputs: HashMap<u32, TransactionOutput>,
}