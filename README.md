## Vitecoin

Simplistic and partial implementation of a Bitcoin node.

Based on https://bitcoin.org/bitcoin.pdf.

✅ What is implemented:

- Checks that a block is valid and adds it to the chain.
- Process all transactions embeded in the block.

❌ What is not implemented:

- Doesn't implement the merkle root as a way to quickly check the validity of a transaction (in this case it doesn't matter because every transaction is processed).
- Doesn't adapt the difficulty of the mining based on the current mining rate.
- Doesn't manage if multiple branches exist in the chain. It properly creates branches, but does not recompute the unspent transaction for each leaf block. Basically if a branch is created, the chain will not work correctly.
- Every cryptographic operation has been replaced by a simple operation suitable for the example.
- Still for simplicity, 32 bit values are used instead of 256 bit values for hash fields.

### Run the project

- Install Rust.
- Run `cargo run`.

This will run `src/main.rs` which simulates the addition of a few blocks in the chain. It also showcase a few examples of invalid blocks.
