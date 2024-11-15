#[derive(Clone)]
pub struct TransactionInput {
    pub prev_transaction_hash: u32,
    pub output_index: u32,
    pub signature: u32,
    pub sequence: u32,
}