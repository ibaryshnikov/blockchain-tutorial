# Part 2 of the blockchain tutorial

A port of https://jeiwan.cc/posts/building-blockchain-in-go-part-2/

## Running the example

```
# run in release mode because proof-of-work takes some time to calculate
cargo run --release
```

In case it takes too long to mine the hash for a block, try reducing target bits from 24 to 16 or more
