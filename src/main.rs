#![allow(unused)]

use block::{block::Block, block_header::BlockHeader};
use constants::{BLOCK_VALUE, STARTING_DIFFICULTY, VERSION};
use node::node::Node;
use transaction::{
    transaction::Transaction, transaction_input::TransactionInput,
    transaction_output::TransactionOutput,
};
use utils::{
    counter::Counter,
    key_registry::{self, KeyRegistry},
};

mod block;
mod constants;
mod node;
mod transaction;
mod utils;

fn main() {
    let mut key_registry = KeyRegistry::new();
    let mut node = Node::new();
    let mut timestamp_counter = Counter::new();
    let mut locktime_counter = Counter::new();

    node.print_unspent_transactions(key_registry.names());

    let bob_key = key_registry.generate("Bob");
    let john_key = key_registry.generate("John");
    let alice_key_1 = key_registry.generate("Alice 1");
    let alice_key_2 = key_registry.generate("Alice 2");
    let alice_key_3 = key_registry.generate("Alice 3");
    let eve_key = key_registry.generate("Eve");

    // Bob is very hyped by this new Vitecoin thing and eagerly mines his first block.
    let bob_coinbase_transaction = Transaction {
        locktime: locktime_counter.next(),
        version: VERSION,
        reward: BLOCK_VALUE,
        inputs: vec![],
        outputs: vec![TransactionOutput {
            recipient_public_key: bob_key,
            value: BLOCK_VALUE,
        }],
    };
    let bob_coinbase_transaction_hash = bob_coinbase_transaction.hash();
    let bob_block = Block {
        header: BlockHeader {
            version: VERSION,
            nonce: 0x0FFFFFFF,
            previous_block_hash: node.get_last_block_hash(),
            timestamp: timestamp_counter.next(),
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        transactions: vec![bob_coinbase_transaction],
    };

    // Bob now wants to pays John 60 units. He sends a transaction to the node to be processed by the next miner:
    // - 60 units go to John
    // - 35 units stay for himself
    // - 5 units go as fee to the miner who will include this transaction (the dude is quite generous)
    let bob_transaction_hash = node.add_transaction(Transaction {
        version: VERSION,
        locktime: locktime_counter.next(),
        reward: 0,
        inputs: vec![TransactionInput {
            prev_transaction_hash: bob_coinbase_transaction_hash,
            output_index: 0,
            sequence: 0,
            // For this simplified example, the signature is the same as the public key.
            // In practice this is slightly more complex. Cryptography may be involved at some point.
            signature: bob_key,
        }],
        outputs: vec![
            TransactionOutput {
                recipient_public_key: john_key,
                value: 60,
            },
            TransactionOutput {
                recipient_public_key: bob_key,
                value: 35,
            },
        ],
    });

    // Alice has been told about the Vitecoin by her good friend Bob and also wants a piece of the cake.
    // She mines her first block and includes all awaiting transactions to get a bit of additional money.
    // Because she's careful, she also decides to split the money accross different keys.
    let alice_coinbase_transaction = Transaction {
        version: VERSION,
        locktime: locktime_counter.next(),
        // The reward value should actually be computed from the transactions being embeded.
        // Here we just hardcode it for convenience.
        reward: BLOCK_VALUE + 5,
        inputs: vec![],
        outputs: vec![
            TransactionOutput {
                recipient_public_key: alice_key_1,
                value: 40,
            },
            TransactionOutput {
                recipient_public_key: alice_key_2,
                value: 65,
            },
        ],
    };
    let alice_coinbase_transaction_hash = alice_coinbase_transaction.hash();
    let mut transactions = vec![alice_coinbase_transaction];
    transactions.append(&mut node.get_awaiting_transactions());

    let alice_block = Block {
        header: BlockHeader {
            version: VERSION,
            nonce: 0x01234567,
            previous_block_hash: node.get_last_block_hash(),
            timestamp: timestamp_counter.next(),
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        transactions,
    };

    // Eve has also heard about the Vitecoin but hasn't quite understood how it works.
    // She repeatedly makes attemps at block mining, but unfortunately makes a mistake every time :(
    let eve_block_1 = Block {
        header: BlockHeader {
            version: VERSION,
            // In reality the difficulty check would be computed on the whole block hash.
            // Here it's computed on the noonce only so it's easily predictable and doesn't take hours to run.
            nonce: 0x7FFFFFFF, // Not enough zeroes!
            previous_block_hash: node.get_last_block_hash(),
            timestamp: timestamp_counter.next(),
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        transactions: vec![]
    };

    let eve_block_2 = Block {
        header: BlockHeader {
            version: VERSION,
            nonce: 0x09876543,
            previous_block_hash: 123456, // She doesn't refer to a valid previous block!
            timestamp: timestamp_counter.next(),
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        transactions: vec![]
    };

    let eve_block_3 = Block {
        header: BlockHeader {
            version: VERSION,
            nonce: 0x09876543,
            previous_block_hash: node.get_last_block_hash(),
            timestamp: 3000000000, // She refers to a time waaaay ahead of network time!
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        transactions: vec![]
    };

    let eve_block_4 = Block {
        header: BlockHeader {
            version: VERSION,
            nonce: 0x09876543,
            previous_block_hash: node.get_last_block_hash(),
            timestamp: timestamp_counter.next(),
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        // At last she gets the header correctly, but alas forgets to include a
        // coinbase transaction to indicate where to store the reward money...
        transactions: vec![]
    };

    // John has not been nice to Alice recently (yes they are together, it was actually Bob who introduced them to each other).
    // So Alice decides that John doesn't need his money anymore and steals the hard drive where he stores his key while he's
    // busy losing his 7th League of Legends game in a row.
    // Since he's also good friend with Alice and starts to pity her, she will give her part of the money (not everything, kindness has its limits).
    // In the process she also merges all of her money on a single new account (it was a pain to keep track of all of them).

    let alice_revenge_block = Block {
        header: BlockHeader {
            version: VERSION,
            nonce: 0x00112233,
            previous_block_hash: node.get_last_block_hash(),
            timestamp: timestamp_counter.next(),
            merkle_root: 0,
            difficulty_target: STARTING_DIFFICULTY,
        },
        transactions: vec![
            // Coinbase transaction
            Transaction {
                version: VERSION,
                locktime: locktime_counter.next(),
                // Reward for mining a block + the money she has on other accounts + money she steals from John - the money she's giving away in the transaction
                reward: BLOCK_VALUE + 105 + 60 - 10 - 1,
                inputs: vec![],
                outputs: vec![
                    TransactionOutput {
                        recipient_public_key: alice_key_3,
                        value: BLOCK_VALUE + 105 + 60 - 10 - 1,
                    }
                ]
            },
            Transaction {
                version: VERSION,
                locktime: locktime_counter.next(),
                reward: 0,
                inputs: vec![
                    TransactionInput {
                        prev_transaction_hash: bob_transaction_hash,
                        output_index: 0,
                        signature: john_key,
                        sequence: 0,
                    },
                    TransactionInput {
                        prev_transaction_hash: alice_coinbase_transaction_hash,
                        output_index: 0,
                        signature: alice_key_1,
                        sequence: 0,
                    },
                    TransactionInput {
                        prev_transaction_hash: alice_coinbase_transaction_hash,
                        output_index: 1,
                        signature: alice_key_2,
                        sequence: 0,
                    }
                ],
                outputs: vec![
                    // Give 10 units to Eve.
                    // Alice doesn't bother specifying other outputs, because the reminder of the transaction will
                    // automatically go to her as fee since she's the one who mines the block.
                    TransactionOutput {
                        recipient_public_key: eve_key,
                        value: 10,
                    },
                    // Leave a single unit of money on John's key as a very petty move.
                    TransactionOutput {
                        recipient_public_key: john_key,
                        value: 1,
                    }
                ]
            }
        ]
    };

    add_block_and_print_state(&mut node, &key_registry, bob_block);
    add_block_and_print_state(&mut node, &key_registry, alice_block);
    add_block_and_print_state(&mut node, &key_registry, eve_block_1);
    add_block_and_print_state(&mut node, &key_registry, eve_block_2);
    add_block_and_print_state(&mut node, &key_registry, eve_block_3);
    add_block_and_print_state(&mut node, &key_registry, eve_block_4);
    add_block_and_print_state(&mut node, &key_registry, alice_revenge_block);
}

fn add_block_and_print_state(node: &mut Node, key_registry: &KeyRegistry, block: Block) {
    print!("\n=> ADDING BLOCK: ");

    match node.add_block(block) {
        Err(error) => println!("{:?}", error),
        Ok(_) => {
            println!("OK");
            node.print_unspent_transactions(key_registry.names());
        }
    };
}
