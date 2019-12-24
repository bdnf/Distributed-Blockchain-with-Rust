use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Index{
    index: usize
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction{
    sender: String,
    recipient: String,
    amount: u128,
    delimiter: Option<Index>
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f64) -> Transaction{
        let amount_string = amount.to_string();
        let (parsed_amount, index) = match amount_string.find(".") {
            Some(index) => {
                let vec = amount_string.split(".").map(String::from).collect::<Vec<_>>();
                let amount_int = vec.join("").to_string().parse::<u128>().expect("Parsing of string to int should not fail");
                let adjusted_index = if (amount.trunc() == 0.0) {
                    println!("Amount was less than 0");
                    //adjust index
                    0
                } else { index };
                (amount_int, Some(Index{index: adjusted_index}))
            },
            None => (amount_string.parse::<u128>().expect("Parsing of string to int should not fail"), None)
        };
        Transaction{
            sender,
            recipient,
            amount: parsed_amount,
            delimiter: index
        }
    }



    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

}


#[cfg(test)]
mod tests {
    /// Test to ensure that hash of the specific transaction is going to be always the same
    use crate::blockchain::transaction::Transaction;
    use sha2::{Sha256, Sha512, Digest};
    use crate::blockchain::block::Block;
    use crate::blockchain::proof::Proof;

    pub fn create_transaction() -> Transaction{
        Transaction::new("Alice".to_string(),"Bob".to_string(), 1.29)
    }

    pub fn hash_string(str: String) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(str);
        let result = hasher.result();
        result[..].to_vec()
    }


    #[test]
    pub fn hash_string_sha256(){

        let tx = create_transaction();
        let str = tx.to_string();
        let result = hash_string(str);

        assert_eq!(result[..], hex!("
        5a6fbeb03b7c8c25ce5e477ddae17772301c43f34a426fe6c569f6909a06c746
    ")[..]);
    }

    #[test]
    pub fn hash_block(){
        let tx = create_transaction();

        let block = Block::new(1, "5a6fbeb03b7c8c25ce5e477ddae17772301c43f34a426fe6c569f6909a06c746".as_bytes().to_vec(), Proof{value: "100".to_string()}, vec![tx]);
        let mut string_to_hash = "".to_string();

        // concatenate transactions to string
        for tx in &block.transactions {
            string_to_hash += &tx.to_string();
        }
        let hash = hash_string(string_to_hash);

        assert_eq!(hash[..], hex!("
            5a6fbeb03b7c8c25ce5e477ddae17772301c43f34a426fe6c569f6909a06c746
        ")[..]);
    }
}

