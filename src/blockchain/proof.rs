use serde::{Deserialize, Serialize};

use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Proof {
    pub value: String
}

impl Proof {
    pub fn valid_proof(&self, transactions: &Vec<Transaction>, lash_hash: &String) -> bool {
//        let guess = unimplemented!();  //all tx to string, concatenate all, concatenate with last_hash
//        let guess_hash: String = self.hash_string_sha256(guess);
//        return self.value == guess_hash
        unimplemented!();
    }

    pub fn proof_of_work(&self, blockchain: &Vec<Block>, open_transactions: &Vec<Transaction>) -> usize {
//        let last_block = blockchain.last();
////        let last_hash = self.hash_block(last_block.unwrap());
////        let mut proof = 0;
////        while !self.valid_proof(open_transactions,&last_hash){
////            proof += 1;
////        }
////        proof
        unimplemented!();
    }
}
