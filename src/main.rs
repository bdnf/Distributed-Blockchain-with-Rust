#[macro_use] extern crate hex_literal;

mod blockchain;

use std::str;
use blockchain::transaction::Transaction;
use sha2::{Sha256, Sha512, Digest};
use blockchain::block::Block;
use blockchain::proof::Proof;
use crate::blockchain::smart_contract::SmartContract;

use std::fs::File;
use std::io::Write;
use std::io::prelude::*;


fn main() {
    println!("Hello, Blockchain!");

    let tx = Transaction::new("Alice".to_string(),"Bob".to_string(), 1.29);
    println!("{:?}", tx);
    println!("{:?}", tx.to_string());

    let str = tx.to_string();

    let mut hasher = Sha256::new();
    hasher.input(str);
    let result = hasher.result();
    println!("{:x}", result);

    assert_eq!(result[..], hex!("
        5a6fbeb03b7c8c25ce5e477ddae17772301c43f34a426fe6c569f6909a06c746
    ")[..]);

    let block = Block::new(1, "5a6fbeb03b7c8c25ce5e477ddae17772301c43f34a426fe6c569f6909a06c746".as_bytes().to_vec(), Proof{value: "100".to_string()}, vec![tx]);
    let mut string_to_hash = "".to_string();
    for tx in &block.transactions {
        string_to_hash += &tx.to_string();
    }
    println!("{:?}", string_to_hash);


    let contract = SmartContract::generate_keys();

    let signature = contract.sign(b"hello, world");
    let string = String::from_utf8_lossy(&signature);
    println!("Signature is {:?}", string);

    assert_ne!(contract.public_key.unwrap().len(), 0);
    //assert_ne!(contract.private_key.unwrap().len(), 0);

}


