use crate::util::string2hash;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::transaction::{TxOut, Txin};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let hash = String::from("0");
        Self {
            timestamp,
            data,
            previous_hash,
            hash,
            nonce: 0,
            transactions: Vec::new(),
        }
    }
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
    pub fn set_timestamp(&mut self, timestamp: u128) {
        self.timestamp = timestamp;
    }
    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }
    pub fn get_data(&self) -> String {
        self.data.clone()
    }
    pub fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }
    pub fn genesis() -> Self {
        Self {
            timestamp: 0,
            data: String::from("Genesis"),
            previous_hash: String::from("0"),
            hash: String::from("0"),
            nonce: 0,
            transactions: Vec::new(),
        }
    }

    pub fn add_coinbase(&mut self,address: String) {
        let mut transaction = Transaction::new();
        let mut txout = TxOut::new();
        let mut txin = Txin::new();
        txout.set_value(50);
        txout.set_pubkey(address);
        transaction.add_input(txin);
        transaction.add_output(txout);
        self.transactions.push(transaction);
    }
    pub fn calculate_hash(&self, nonce: u32) -> String {
        let data = format!("{}{}{}{}", self.timestamp, self.data, self.previous_hash, nonce);
        string2hash(data)
    }
    pub fn mine_block(&mut self,difficulty: u32,address: String, txid: u64) {
        let mut nonce = 0;
        let prefix = "0".repeat(difficulty as usize);
        loop {
            let hash = self.calculate_hash(nonce);
            if hash.starts_with(&prefix) {
                self.hash = hash;
                break;
            }
            nonce += 1;
        }
        self.nonce = nonce;
        self.add_coinbase(address);
        self.set_txid(txid);
    }
    pub fn set_txid(&mut self, txid: u64) {
        let mut txid = txid+1;
        for transaction in self.transactions.iter_mut() {
            transaction.set_txid(txid);
            txid += 1;
        }
    }
}