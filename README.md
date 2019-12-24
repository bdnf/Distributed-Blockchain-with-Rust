# Distributed Blockchain in Rust

Simple implementation of distributed data structure.

Single block hold `Transaction` at the moment. 

For block to hold another object just change the corresponding class.


## Run tests with

The whole implementation
```
cargo test
```
Only blockchain part
```
cargo test blockchain
```
Only specific module
```
cargo test transaction
```