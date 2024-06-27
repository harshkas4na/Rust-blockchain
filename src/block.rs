use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
// use std::fmt;

const INITIAL_DIFFICULTY: u64 = 3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub nonce:u64,
    pub difficulty:u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            data,
            difficulty: INITIAL_DIFFICULTY,
            nonce: 0,
            previous_hash: previous_hash.clone(),
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let block_data = format!("{}{}{}{}{}{}", self.index, self.timestamp,self.nonce,self.difficulty, self.data, self.previous_hash);
        hasher.update(block_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
