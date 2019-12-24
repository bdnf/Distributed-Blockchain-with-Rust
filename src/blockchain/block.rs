use serde::{Deserialize, Serialize};

use crate::blockchain::proof::Proof;
use crate::blockchain::transaction::Transaction;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Block{
    pub index: u64,
    pub previous_hash: Vec<u8>,
    pub timestamp: u128,
    pub proof: Proof,
    pub transactions: Vec<Transaction>,
}

// used to create genesis block
impl Default for Block{
    fn default() -> Block{
        Block{
            index: 0,
            previous_hash: Vec::new(),
            transactions: Vec::new(),
            proof: Proof{value: "100".to_string()},
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Current time cannot be null").as_millis(),
        }
    }
}
impl Block{
    pub fn new(index: u64,
               previous_hash: Vec<u8>,
               proof: Proof,
               transactions: Vec<Transaction>) -> Block{
        Block{
            index,
            previous_hash,
            transactions,
            proof,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Current time cannot be null").as_millis(),
        }
    }

    pub fn all_transactions_as_string(&self) -> String {
        let mut string_to_hash = "".to_string();

        // concatenate transactions to string
        for tx in &self.transactions {
            string_to_hash += &tx.to_string();
        }
        string_to_hash
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
