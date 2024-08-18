use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Block {
    index: u64,
    timestamp: u64,
    txn_data: String,
    previous_block_hash: String,
    current_block_hash: String,
}

impl Block {
    // 1. Create a new block.
    pub fn new(index: u64, txn_data: String, previous_block_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let current_block_hash = Block::calculate_hash(index, timestamp, &txn_data, &previous_block_hash);

        Block {
            index,
            timestamp,
            txn_data,
            previous_block_hash,
            current_block_hash,
        }
    }

    // 2. Generate a hash for the block.
    fn calculate_hash(index: u64, timestamp: u64, txn_data: &str, previous_block_hash: &str) -> String {
        let to_hash = format!("{}{}{}{}", index, timestamp, txn_data, previous_block_hash);
        format!("{:x}", md5::compute(to_hash.as_bytes()))
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    // 1. Create a new blockchain.
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0000000000000000000".to_string());

        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    // 2. Add a new block to the blockchain.
    pub fn add_block(&mut self, txn_data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, txn_data, previous_block.current_block_hash.clone());
        self.blocks.push(new_block);
    }
}