use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Block::new_genesis_block()],
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().expect("Should have at least one block");
        let new_block = Block::new(data, &prev_block.hash);
        self.blocks.push(new_block)
    }
}
