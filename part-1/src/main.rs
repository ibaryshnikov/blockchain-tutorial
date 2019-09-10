use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

fn get_hash_for_block(prev_block_hash: &str, data: &str, timestamp: i64) -> String {
    let timestamp_bytes = timestamp.to_le_bytes();
    let headers = [
        prev_block_hash.as_bytes(),
        data.as_bytes(),
        &timestamp_bytes[..],
    ]
    .concat();
    sha256(&headers)
}

struct Block {
    timestamp: i64,
    data: String,
    prev_block_hash: String,
    hash: String,
}

impl Block {
    fn new(data: &str, prev_block_hash: &str) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp_nanos();

        Block {
            timestamp,
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            hash: get_hash_for_block(prev_block_hash, data, timestamp),
        }
    }
    fn new_genesis_block() -> Self {
        Block::new("Genesis Block", "")
    }
}

struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> Self {
        BlockChain {
            blocks: vec![Block::new_genesis_block()],
        }
    }
}

impl BlockChain {
    fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().expect("Should have at least one block");
        let new_block = Block::new(data, &prev_block.hash);
        self.blocks.push(new_block)
    }
}

fn main() {
    let mut bc = BlockChain::new();

    // all the texts are from https://jeiwan.cc/posts/building-blockchain-in-go-part-1/

    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");

    for block in bc.blocks {
        println!("Prev. hash: {}", block.prev_block_hash);
        println!("Data: {}", block.data);
        println!("Hash: {}\n", block.hash);
    }
}
