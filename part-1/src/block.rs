use chrono::prelude::*;

use crate::hash::sha256;

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

pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp_nanos();

        Block {
            timestamp,
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            hash: get_hash_for_block(prev_block_hash, data, timestamp),
        }
    }

    pub fn new_genesis_block() -> Self {
        Block::new("Genesis Block", "")
    }
}
