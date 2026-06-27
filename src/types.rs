use bincode::{Encode, config};

use crate::{crypto::Hash, util};

#[derive(Debug, Encode)]
pub struct Block {
    prev_hash: Hash,
    content: String,
    timestamp: u64,
}

impl Block {
    pub fn new(prev_hash: Hash, content: String) -> Self {
        Self {
            prev_hash: prev_hash,
            content: content,
            timestamp: util::now_secs(),
        }
    }

    pub fn prev_hash(&self) -> Hash {
        self.prev_hash
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn hash(&self) -> Hash {
        let bytes = bincode::encode_to_vec(self, config::standard()).unwrap();
        Hash::hash(&bytes)
    }
}

pub struct HashChain {
    blocks: Vec<Block>,
}

impl HashChain {
    pub fn new() -> Self {
        let genesis_block = Block::new(Hash::zero(), "hello world!".to_string());
        Self {
            blocks: vec![genesis_block],
        }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
