use crate::blockchain::block::Block;
use crate::blockchain::verifier::Verifier;

pub mod block;
pub mod proof;
pub mod transaction;
pub mod verifier;
pub mod smart_contract;


#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    verificator: Verifier,
}

impl Default for Blockchain {
    fn default() -> Blockchain {
        Blockchain {
            chain: vec![Block{
                ..Default::default()
            }],  // genesis block
            verificator: Verifier{}
        }
    }
}

impl Blockchain {
    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn mine_block(self) -> Self{
        unimplemented!()
    }

    pub fn verify_all_transactions(&self){
        unimplemented!()
    }

    pub fn verify_chain(&self) -> bool{
        let GENESIS_INDEX = 0;
        for (index, block) in self.chain.iter().enumerate()  {
            if index == 0 { continue }
            let previous_block = self.chain[index-1].to_string();
            if block.previous_hash != self.verificator.hash_string_sha256(previous_block) {
                if block.index == GENESIS_INDEX {
                    continue
                }
                return false
            }
        }
        return true
    }

    pub fn get_chain(&self) {
        // return blockchain as JSON
    }

}

#[cfg(test)]
mod tests {
    use crate::blockchain::block::Block;
    use crate::blockchain::Blockchain;
    use crate::blockchain::proof::Proof;
    use crate::blockchain::transaction::Transaction;

    #[test]
    pub fn verity_chain(){
        let tx = Transaction::new("Alice".to_string(),"Bob".to_string(), 1.29);
           let mut blockchain: Blockchain = Blockchain{
            ..Default::default()
        };

        let genesis_block = blockchain.last_block().unwrap();
        let genesis_hash = blockchain.verificator.hash_block(genesis_block);
        let block = Block::new(1, genesis_hash, Proof{value: "100".to_string()}, vec![tx]);

        blockchain.chain.push(block);

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.verify_chain(), true);

        let tx2 = Transaction::new("Alice".to_string(),"Bob".to_string(), 1.23);
        let previous_block = blockchain.last_block().unwrap();
        let previous_hash = blockchain.verificator.hash_block(previous_block);
        let block2 = Block::new(1, previous_hash, Proof{value: "100".to_string()}, vec![tx2]);
        blockchain.chain.push(block2);

        assert_eq!(blockchain.chain.len(), 3);
        assert_eq!(blockchain.verify_chain(), true);

        // try to hack
        let tx3 = Transaction::new("Alice".to_string(),"Bob".to_string(), 1.23);

        let random_hash = "5a6fbeb03b7c8c25ce5e477ddacee17772301c43f34a426fe6c569f6909a06c746".as_bytes().to_vec();
        let block3 = Block::new(1, random_hash, Proof{value: "100".to_string()}, vec![tx3]);
        blockchain.chain.push(block3);

        assert_eq!(blockchain.chain.len(), 4);
        assert_eq!(blockchain.verify_chain(), false);


    }
}

