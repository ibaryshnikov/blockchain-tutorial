use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::proofofwork::ProofOfWork;

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: i64,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp_nanos();

        let mut block = Block {
            timestamp,
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            hash: String::new(),
            nonce: 0,
        };

        let pow = ProofOfWork::new(&block);
        let (nonce, hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        block
    }
    pub fn new_genesis_block() -> Self {
        Block::new("Genesis Block", "")
    }
}
