use crate::util::string2hash;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
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
        }
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
        }
    }

    pub fn calculate_hash(&self, nonce: u32) -> String {
        let data = format!("{}{}{}{}", self.timestamp, self.data, self.previous_hash, nonce);
        string2hash(data)
    }
    pub fn mine_block(&mut self, difficulty: u32) {
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
    }
}