use crate::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(
            0,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            "Genesis Block".to_string(),
            "0".to_string(),
        );
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();

        //writing a logic to add a block after a computation of proof of work
        let mut new_block = Block::new(
            previous_block.index + 1,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            data.clone(),
            previous_block.hash.clone(),
        );
        loop{
            let prevtimestamp=previous_block.timestamp;
            let currenttimestamp=new_block.timestamp;
            let timediff=currenttimestamp-prevtimestamp;

            if new_block.difficulty < 1 {
                new_block.difficulty=1;
            }
            if timediff > 1 {
                new_block.difficulty = previous_block.difficulty - 1;
            } else {
                new_block.difficulty = previous_block.difficulty + 1;
            }


            if new_block.hash.starts_with("0".repeat(new_block.difficulty as usize).as_str()) {
                self.chain.push(new_block);
                break;
            } else {
                new_block.nonce += 1;
                new_block.hash = new_block.calculate_hash();
            }
        }


        
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
