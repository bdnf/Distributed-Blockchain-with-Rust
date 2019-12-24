use sha2::{Sha256, Sha512, Digest};
use crate::blockchain::block::Block;

#[derive(Debug)]
pub struct Verifier{

}

impl Verifier {
    pub fn hash_block(&self, block: &Block) -> Vec<u8> {
        return self.hash_string_sha256(block.to_string())
    }

    pub fn hash_string_sha256(&self, str: String) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(str);
        let result = hasher.result();
        result[..].to_vec()
    }

    pub fn verify_one_transaction(&self){
        unimplemented!()
    }

    pub fn verify_all_transactions(&self){
        unimplemented!()
    }


}