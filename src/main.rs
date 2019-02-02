#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate crypto;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: i64
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Block {
    index: usize,
    timestamp: Duration,
    transactions: Vec<Transaction>,
    proof: usize,
    previous_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Blockchain {
    current_transactions: Vec<Transaction>,
    chain: Vec<Block>,
    //nodes: Vec<String>, // It should be set() in python
}

impl Blockchain {
    fn new_block(&mut self, proof: usize, previous_hash: String) -> Block {
        // Returns a new block
        let tm = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let block = Block {
            index: self.chain.len() + 1,
            timestamp: tm,
            transactions: self.current_transactions.clone(),
            proof: proof,
            previous_hash: previous_hash,
        };
        self.chain.push(block.clone());
        self.current_transactions = Vec::new();
        block
    }
    fn new_transaction(&mut self, sender: String, recipient: String, amount: i64) {
        let tx = Transaction { sender: sender, recipient: recipient, amount: amount };
        self.current_transactions.push(tx);
    }
    fn hash(block: &str) -> String{
        // Creates a SHA-256 hash of a Block
        let ser = serde_json::to_string(&block).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(ser.as_str());
        hasher.result_str()
    }

    fn last_block(&self) -> &Block {
        let length = self.chain.len();
        // self.chain.get(length - 1)
        &self.chain[length - 1]
    }
}

fn main() {
    let mut blockchain = Blockchain {
        current_transactions:  Vec::new(),
        chain: Vec::new(),
    };
    
    blockchain.new_transaction(String::from("coni"), String::from("ceni"), 152);
    blockchain.new_transaction(String::from("AA"), String::from("BB"), 44);
    blockchain.new_transaction(String::from("CC"), String::from("DD"), 94);
    blockchain.new_block(987654321, String::from("previous hash - 1"));

    blockchain.new_transaction(String::from("mdemir"), String::from("ali"), 22);
    blockchain.new_block(987654321, String::from("previous hash - 1 - 1"));

    let ser = serde_json::to_string(&blockchain).unwrap();
    println!("{}", ser);
    
    println!("{:?}", Blockchain::hash(ser.as_str()));
}
