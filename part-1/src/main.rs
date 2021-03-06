mod block;
mod blockchain;
mod hash;

use blockchain::BlockChain;

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
