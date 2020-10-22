use sled::{Db, Error as DbError};
use sled::transaction::TransactionResult;

use crate::block::Block;

const LAST_HASH: &[u8] = b"last_hash";

pub struct BlockChain<'a> {
    pub tip: String,
    pub db: &'a Db,
}

impl<'a> BlockChain<'a> {
    pub fn new(db: &'a Db) -> Result<Self, DbError> {
        let result: TransactionResult<String> = db
            .transaction(|db| {
                let tip = match db.get(LAST_HASH)? {
                    Some(data) => {
                        String::from_utf8(data.to_vec()).expect("Should create a hash from buffer")
                    }
                    None => {
                        println!("No existing blockchain found. Creating a new one...");
                        let genesis = Block::new_genesis_block();
                        let serialized =
                            rmp_serde::to_vec(&genesis).expect("Should serialize a block");
                        db.insert(genesis.hash.as_bytes(), serialized)?;
                        db.insert(LAST_HASH, genesis.hash.as_bytes())?;
                        genesis.hash
                    }
                };
                Ok(tip)
            });
        let tip = result.expect("Should initialize the blockchain");
        Ok(BlockChain { tip, db })
    }

    pub fn add_block(&mut self, data: &str) -> Result<(), DbError> {
        let last_hash_bytes = self
            .db
            .get(LAST_HASH)?
            .ok_or_else(|| DbError::Unsupported("Last hash not found".to_string()))?;
        let last_hash =
            String::from_utf8(last_hash_bytes.to_vec()).expect("Should create a hash from buffer");

        let block = Block::new(data, &last_hash);

        let result: TransactionResult<()> = self.db
            .transaction(|db| {
                let serialized = rmp_serde::to_vec(&block).expect("Should serialize a block");
                db.insert(block.hash.as_bytes(), serialized)?;
                db.insert(LAST_HASH, block.hash.as_bytes())?;
                Ok(())
            });
        result.expect("Should write a new block to db");

        self.tip = block.hash;

        Ok(())
    }
}

pub struct BlockChainIterator<'a> {
    pub current_hash: String,
    pub db: &'a Db,
}

impl BlockChainIterator<'_> {
    pub fn next(&mut self) -> Result<Option<Block>, DbError> {
        match self.db.get(self.current_hash.as_bytes())? {
            Some(data) => {
                let block: Block =
                    rmp_serde::from_read(&data[..]).expect("Should deserialize a block");
                self.current_hash = block.prev_block_hash.clone();
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
}
