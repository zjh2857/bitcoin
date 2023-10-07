use crate::block::Block;
use crate::transaction::Transaction;
use crate::transaction::TxOut;
use crate::transaction::Txin;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ed25519::keypair;
use crypto::ed25519::signature;

#[derive(Debug, Clone)]
pub struct Blockchain {
    difficulty: u32,
    db : sled::Db,
    reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::genesis();
        let db = sled::open("blockchain").unwrap();
        match db.get("latest") {
            Ok(Some(_)) => {
                
            }
            Ok(None) => {
                println!("Blockchain does not exist. Creating a new one...");
                db.insert("latest".to_string(), bincode::serialize(&genesis).unwrap()).unwrap();
                db.insert(genesis.get_hash(), bincode::serialize(&genesis).unwrap()).unwrap();
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        Self {
            difficulty: 1,
            db,
            reward: 50,
        }
    }

    pub fn check_signature(&self,from:String, to: String, amount: u64, signature:String) -> bool {
        //TODO
        true
    }

    pub fn add_transaction(&mut self, from:String, to: String, signature:String ,amount: u64) {

        if self.check_signature(from.clone(), to.clone(), amount, signature.clone()) == false {
            println!("Invalid signature");
            return;
        }

        let last:Block = bincode::deserialize(&self.db.get("latest").unwrap().unwrap()).unwrap();
        let mut block = Block::new(String::from("Transaction"), last.get_hash());
        let mut transaction = Transaction::new();
        let utxo = self.find_utxo(from.clone());
        let mut inamout = 0;
        for tx in utxo {
            for txout in tx.get_txouts() {
                let mut txin = Txin::new();
                txin.set_value(txout.get_value());
                inamout += txout.get_value();
                txin.set_pubkey(txout.pubkey);
                txin.set_signature(signature.clone());
                transaction.add_input(txin);
            }
        }
        if inamout < amount {
            println!("Not enough money");
            return;
        }
        let mut txout = TxOut::new();
        txout.set_value(amount);
        txout.set_pubkey(to);
        transaction.add_output(txout);

        let mut txout = TxOut::new();
        txout.set_pubkey(from.clone());
        txout.set_value(inamout-amount+self.reward);
        transaction.add_output(txout);
        block.add_transaction(transaction);

        block.mine_block(self.difficulty);

        self.db.insert(block.get_hash(), bincode::serialize(&block).unwrap()).unwrap();
        self.db.insert("latest".to_string(), bincode::serialize(&block).unwrap()).unwrap();

    }

    pub fn find_utxo(&self, pubkey: String) -> Vec<Transaction> {
        let mut utxo: Vec<Transaction> = Vec::new();
        let mut last:Block = bincode::deserialize(&self.db.get("latest").unwrap().unwrap()).unwrap();
        let mut previous_block:Block = bincode::deserialize(&self.db.get(last.get_previous_hash()).unwrap().unwrap()).unwrap();
        while previous_block.get_hash() != "0".to_string() {
            for transaction in previous_block.get_transactions() {
                for txout in transaction.get_txouts() {
                    if txout.pubkey == pubkey { // TODO: check the signature 
                        utxo.push(transaction.clone());
                    }
                }
            }
            last = previous_block;
            previous_block = bincode::deserialize(&self.db.get(last.get_previous_hash()).unwrap().unwrap()).unwrap();
        }
        utxo
    }
    pub fn add_block(&mut self, data: String) {
        let test = self.db.get("latest").unwrap().unwrap();
        let test = bincode::deserialize::<Block>(&test);
        match test {
            
            Ok(_) => {
                println!("Blockchain already exists.");
                println!("{}",data);
            }
            Err(err) => {
                println!("111: {}", err);
            }
        }
        let previous: Block = bincode::deserialize::<Block>(&self.db.get("latest").unwrap().unwrap()).unwrap();
        let previous_hash = previous.get_hash();
        let mut block = Block::new(data, previous_hash);
        let difficulty: u32 = self.difficulty;
        block.mine_block(difficulty);
        
        self.db.insert(block.get_hash(), bincode::serialize(&block).unwrap()).unwrap();
        self.db.insert("latest".to_string(), bincode::serialize(&block).unwrap()).unwrap();
    }
    pub fn is_valid(&self) -> bool {
        let mut last:Block = bincode::deserialize(&self.db.get("latest").unwrap().unwrap()).unwrap();
        let mut previous_block:Block = bincode::deserialize(&self.db.get(last.get_previous_hash()).unwrap().unwrap()).unwrap();
        let difficulty: u32 = self.difficulty;
        let prefix: String = "0".repeat(difficulty as usize);
        while previous_block.get_hash() != "0".to_string() {
            let hash = last.calculate_hash(last.get_nonce());
            if !hash.starts_with(&prefix) || hash != last.get_hash(){
                print!("Invalid block");
                return false;
            }
            last = previous_block;
            previous_block = bincode::deserialize(&self.db.get(last.get_previous_hash()).unwrap().unwrap()).unwrap();
        }
        true
    }
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }
    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.difficulty = difficulty;
    }

    pub fn show(&self) {
        let mut last:Block = bincode::deserialize(&self.db.get("latest").unwrap().unwrap()).unwrap();
        let mut previous_block:Block = bincode::deserialize(&self.db.get(last.get_previous_hash()).unwrap().unwrap()).unwrap();
        let difficulty: u32 = self.difficulty;
        let prefix: String = "0".repeat(difficulty as usize);
        while previous_block.get_hash() != "0".to_string() {
            println!("{:?}",last);
            last = previous_block;
            previous_block = bincode::deserialize(&self.db.get(last.get_previous_hash()).unwrap().unwrap()).unwrap();
        }
    }
}
