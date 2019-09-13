# Part 3 of the blockchain tutorial

A port of https://jeiwan.cc/posts/building-blockchain-in-go-part-3/

## Running the example

```
# print help
cargo run -- --help
# print all blocks
cargo run -- printchain
# add new block
# run in release mode because proof-of-work takes some time to calculate
cargo run --release -- addblock --data block_data
```

In case it takes too long to mine the hash for a block, try reducing target bits from 24 to 16 or more
