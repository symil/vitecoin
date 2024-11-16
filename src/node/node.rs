use super::node_error::NodeError;
use crate::{
    block::{block::Block, block_header::BlockHeader, block_wrapper::BlockWrapper},
    constants::{
        BLOCK_VALUE, GENESIS_BLOCK_HASH, MAX_AHEAD_OF_TIME_TIMESTAMP_SECS, STARTING_DIFFICULTY,
    },
    transaction::{
        transaction::Transaction, transaction_input::TransactionInput, transaction_output::TransactionOutput, unspent_transaction::UnspentTransaction
    },
};
use std::{collections::HashMap, time::SystemTime};

#[derive(Default)]
pub struct Node {
    blocks: HashMap<u32, BlockWrapper>,
    unspent_transactions: HashMap<u32, UnspentTransaction>,
    current_difficulty: u32,
    last_block_hash: u32,
    transaction_pool: HashMap<u32, Transaction>,
}

impl Node {
    pub fn new() -> Self {
        let mut node = Self {
            blocks: HashMap::default(),
            unspent_transactions: HashMap::default(),
            current_difficulty: STARTING_DIFFICULTY,
            last_block_hash: GENESIS_BLOCK_HASH,
            transaction_pool: HashMap::new(),
        };
        let genesis_block_header = BlockHeader::genesis();

        node.blocks.insert(
            GENESIS_BLOCK_HASH,
            BlockWrapper::from_header(genesis_block_header),
        );

        node
    }

    pub fn add_block(&mut self, block: Block) -> Result<u32, NodeError> {
        let block_hash = block.hash();
        let prev_block_wrapper = self
            .blocks
            .get(&block.header.previous_block_hash)
            .ok_or(NodeError::InvalidPrevBlockHash)?;

        if block.header.difficulty_target != self.current_difficulty {
            return Err(NodeError::InvalidDifficulty);
        }

        if !self.check_hash_difficulty(block_hash) {
            return Err(NodeError::InvalidDifficulty);
        }

        if block.header.timestamp <= prev_block_wrapper.header.timestamp {
            return Err(NodeError::InvalidTimestamp);
        }

        if block.header.timestamp > self.get_current_time() + MAX_AHEAD_OF_TIME_TIMESTAMP_SECS {
            return Err(NodeError::InvalidTimestamp);
        }

        // TODO: check for validity of merkle root

        let mut anounced_reward = 0;
        let mut actual_reward = BLOCK_VALUE;
        let mut outputs_to_add: Vec<(u32, u32, TransactionOutput)> = vec![];
        let mut inputs_to_remove: Vec<&TransactionInput> = vec![];

        if block.transactions.len() < 1 {
            // Coinbase transaction is missing
            return Err(NodeError::InvalidCoinbaseTransaction);
        }

        for (index, transaction) in block.transactions.iter().enumerate() {
            let is_coinbase_transaction = index == 0;
            let mut input_sum = 0;
            let mut output_sum = 0;
            let transaction_hash = transaction.hash();

            if is_coinbase_transaction {
                input_sum += transaction.reward;
                anounced_reward = transaction.reward;
            } else if transaction.reward != 0 {
                return Err(NodeError::InvalidTransactionReward);
            }

            for input in &transaction.inputs {
                let prev_transaction = self
                    .unspent_transactions
                    .get(&input.prev_transaction_hash)
                    .ok_or(NodeError::InvalidTransactionInputHash)?;

                let prev_output = prev_transaction
                    .unspent_outputs
                    .get(&input.output_index)
                    .ok_or(NodeError::InvalidTransactionInputIndex)?;

                // In reality this check would be much more complex and involve cryptography
                if input.signature != prev_output.recipient_public_key {
                    return Err(NodeError::InvalidTransactionInputSignature);
                }

                input_sum += prev_output.value;

                inputs_to_remove.push(&input);
            }

            for (output_index, output) in transaction.outputs.iter().enumerate() {
                output_sum += output.value;

                outputs_to_add.push((transaction_hash, output_index as u32, output.clone()));
            }

            if output_sum > input_sum {
                return Err(NodeError::InvalidTransactionBalance);
            }

            let fee = input_sum - output_sum;

            actual_reward += fee;

            if is_coinbase_transaction && fee != 0 {
                // The coinbase transaction should not have any fee
                return Err(NodeError::InvalidCoinbaseTransaction);
            }
        }

        if actual_reward != anounced_reward {
            // The coinbase transaction does not match the block's content
            return Err(NodeError::InvalidCoinbaseTransaction);
        }

        // At this point the block is valid

        // Remove spent transactions
        for input in inputs_to_remove {
            let transaction = self
                .unspent_transactions
                .get_mut(&input.prev_transaction_hash)
                .unwrap();

            transaction.unspent_outputs.remove(&input.output_index);

            if transaction.unspent_outputs.is_empty() {
                self.unspent_transactions
                    .remove(&input.prev_transaction_hash);
            }
        }

        // Add new unspent transactions
        for (transaction_hash, output_index, output) in outputs_to_add {
            let transaction = self
                .unspent_transactions
                .entry(transaction_hash)
                .or_insert_with(|| UnspentTransaction::new(transaction_hash));

            transaction.unspent_outputs.insert(output_index, output);
            self.transaction_pool.remove(&transaction_hash);
        }

        // Register the new block
        self.blocks
            .insert(block_hash, BlockWrapper::from_header(block.header));

        // Specify that the new block is the successor of the previous one. If there was already one, this creates a new "branch" in the chain.
        self.blocks
            .get_mut(&block_hash)
            .unwrap()
            .next_blocks_hashes
            .push(block_hash);

        self.last_block_hash = block_hash;

        // TODO: adjust block difficulty

        Ok(block_hash)
    }

    pub fn get_last_block_hash(&self) -> u32 {
        self.last_block_hash
    }

    pub fn get_awaiting_transactions(&self) -> Vec<Transaction> {
        Vec::from_iter(self.transaction_pool.values().into_iter().cloned())
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> u32 {
        let hash = transaction.hash();

        self.transaction_pool.insert(transaction.hash(), transaction);

        hash
    }

    fn check_hash_difficulty(&self, hash: u32) -> bool {
        hash.leading_zeros() >= self.current_difficulty
    }

    fn get_current_time(&self) -> u32 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }

    pub fn print_unspent_transactions(&self, owners: &HashMap<u32, String>) {
        if self.unspent_transactions.is_empty() {
            println!("<Nobody has any money>");
        }

        for transaction in self.unspent_transactions.values() {
            for output in transaction.unspent_outputs.values() {
                let owner = match owners.get(&output.recipient_public_key) {
                    Some(name) => name,
                    None => &output.recipient_public_key.to_string(),
                };

                println!("{}: {} units", owner, output.value);
            }
        }
    }
}
