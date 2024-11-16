#[derive(Debug)]
pub enum NodeError {
    InvalidPrevBlockHash,
    InvalidDifficulty,
    InvalidTimestamp,
    InvalidCoinbaseTransaction,
    InvalidTransactionInputHash,
    InvalidTransactionInputIndex,
    InvalidTransactionInputSignature,
    InvalidTransactionBalance,
    InvalidTransactionReward,
}