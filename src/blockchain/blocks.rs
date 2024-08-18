use std::time::{SystemTime, UNIX_EPOCH};
use super::pos::{PosMechanism, Validator};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub txn_data: String,
    pub previous_block_hash: String,
    pub current_block_hash: String,
    pub validator: String,
}

impl Block {
    // 1. Create a new block.
    pub fn new(index: u64, txn_data: String, previous_block_hash: String, validator: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let current_block_hash = Block::calculate_hash(index, timestamp, &txn_data, &previous_block_hash);

        Block {
            index,
            timestamp,
            txn_data,
            previous_block_hash,
            current_block_hash,
            validator,
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
    pub pos_mechanism: PosMechanism,
}

impl Blockchain {
    // 1. Create a new blockchain.
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0000000000000000000".to_string(), "Genesis Validator".to_string());
        let finality_threshold: u64 = 85;
        let penalty_percentage: u64 = 55;
        let pos_mechanism = PosMechanism::new(finality_threshold, penalty_percentage);

        Blockchain {
            blocks: vec![genesis_block],
            pos_mechanism,
        }
    }

    // 2. Add a new block to the blockchain.
    pub fn add_block(&mut self, txn_data: String) {
        let previous_block = self.blocks.last().unwrap();
        let validator = self.pos_mechanism.select_validator().unwrap_or("No Validator".to_string());
        let new_block = Block::new(
            previous_block.index + 1,
            txn_data,
            previous_block.current_block_hash.clone(),
            validator
        );
        self.blocks.push(new_block);
    }

    pub fn add_validators(&mut self, address: String, stake: u64) {
        self.pos_mechanism.add_validator(address, stake);
    }
}