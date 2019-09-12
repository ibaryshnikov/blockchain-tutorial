use num_bigint::BigInt;
use num_traits::{Num, One};

use crate::block::Block;
use crate::hash::sha256;

const TARGET_BITS: i32 = 24;
const MAX_NONCE: i64 = std::i64::MAX;

pub struct ProofOfWork<'a> {
    block: &'a Block,
    target: BigInt,
}

impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a Block) -> Self {
        let target = BigInt::one() << (256 - TARGET_BITS) as usize;
        ProofOfWork { block, target }
    }
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        [
            self.block.prev_block_hash.as_bytes(),
            self.block.data.as_bytes(),
            format!("{:#x}", self.block.timestamp).as_bytes(),
            format!("{:#x}", i64::from(TARGET_BITS)).as_bytes(),
            format!("{:#x}", nonce).as_bytes(),
        ]
        .concat()
    }

    pub fn run(self) -> (i64, String) {
        let mut hash = "".to_string();
        let mut nonce = 0;
        println!("Mining the block containing {}", self.block.data);
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = sha256(&data);
            print!("\r{}", hash);
            let hash_int = BigInt::from_str_radix(&hash, 16).expect("Should be a valid number");
            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }

        println!("\n");

        (nonce, hash)
    }

    pub fn validate(self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = sha256(&data);
        let hash_int = BigInt::from_str_radix(&hash, 16).expect("Should be a valid number");
        hash_int < self.target
    }
}
