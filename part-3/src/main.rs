use sled::{Error as DbError};
use structopt::StructOpt;

mod block;
mod blockchain;
mod cli;
mod hash;
mod proofofwork;

use blockchain::{BlockChain, BlockChainIterator};
use cli::{Command, Options};
use proofofwork::ProofOfWork;

fn main() -> Result<(), DbError> {
    let options = Options::from_args();

    let db = sled::open("blockchain_db")?;
    let mut bc = BlockChain::new(&db)?;

    match options.command {
        Command::AddBlock { data } => {
            bc.add_block(&data)?;
        }
        Command::PrintChain => {
            let mut bc_iterator = BlockChainIterator {
                current_hash: bc.tip.clone(),
                db: &db,
            };
            while let Some(block) = bc_iterator.next()? {
                println!("Prev. hash: {}", block.prev_block_hash);
                println!("Data: {}", block.data);
                println!("Hash: {}", block.hash);
                let pow = ProofOfWork::new(&block);
                println!("PoW: {}\n", pow.validate());
            }
        }
    }

    Ok(())
}
