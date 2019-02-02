#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate crypto;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

/// Represents a transaction
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: i64
}

/// Represents a Block
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Block {
    index: usize,
    timestamp: Duration,
    transactions: Vec<Transaction>,
    proof: usize,
    previous_hash: String,
}

/// Represents the Blockchain
/// Store current transactions for the next Block
/// and store the chain
#[derive(Deserialize, Serialize, Debug)]
struct Blockchain {
    #[serde(skip_serializing)]
    current_transactions: Vec<Transaction>,
    chain: Vec<Block>
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            current_transactions: Vec::new(),
            chain: Vec::new(),
        };

        // Create the genesis block
        blockchain.new_block(100, String::from("1"));
        blockchain
    }

    /// Create a new Block and adds it to the chain list
    /// Set current transactions as transactions for the Block
    /// and empty current transactions list for the next Block 
    /// Returns a new block
    fn new_block(&mut self, proof: usize, previous_hash: String) -> Block {
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

    /// Adds a new transaction to the list of current transactions
    fn new_transaction(&mut self, sender: String, recipient: String, amount: i64) {
        let tx = Transaction { sender: sender, recipient: recipient, amount: amount };
        self.current_transactions.push(tx);
    }

    /// Creates a SHA-256 hash of a Block
    fn hash(block: &str) -> String{
        let ser = serde_json::to_string(&block).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(ser.as_str());
        hasher.result_str()
    }

    /// Returns the last Block in the chain
    fn last_block(&self) -> &Block {
        let length = self.chain.len();
        // self.chain.get(length - 1)
        &self.chain[length - 1]
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    
    blockchain.new_transaction(String::from("coni"), String::from("ceni"), 152);
    blockchain.new_transaction(String::from("AA"), String::from("BB"), 44);
    blockchain.new_transaction(String::from("CC"), String::from("DD"), 94);
    blockchain.new_transaction(String::from("mdemir"), String::from("ali"), 22);

    blockchain.new_block(987654321, String::from("previous hash - 1 - 1"));

    let ser = serde_json::to_string_pretty(&blockchain).unwrap();
    println!("{}", ser);

    //println!("{:?}", Blockchain::hash(ser.as_str()));
}
