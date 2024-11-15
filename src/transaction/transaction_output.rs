#[derive(Clone)]
pub struct TransactionOutput {
    pub value: u64,
    pub recipient_public_key: u32,
}