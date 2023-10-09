use crate::block::Block;
use crate::transaction::Transaction;
use crate::transaction::TxOut;
use crate::transaction::Txin;


#[derive(Debug, Clone)]
pub struct Blockchain {
    difficulty: u32,
    db : sled::Db,
    reward: u64,
    utxo: UtxoSet,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::genesis();
        let db = sled::open("blockchain").unwrap();
        let utxo = UtxoSet::new();
        match db.get("latest") {
            Ok(Some(_)) => {
                
            }
            Ok(None) => {
                println!("Blockchain does not exist. Creating a new one...");

                db.insert("latest".to_string(), bincode::serialize(&genesis).unwrap()).unwrap();
                db.insert(genesis.get_hash(), bincode::serialize(&genesis).unwrap()).unwrap();
                let zero:u64 = 0;
                db.insert("latesttxid".to_string(), bincode::serialize(&zero).unwrap()).unwrap();

            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        Self {
            difficulty: 1,
            db,
            reward: 50,
            utxo: utxo,
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

        let utxo = self.utxo.find_spendable(from.clone());
        let mut inamout = 0;
        let mut transaction = Transaction::new();
        let txid = bincode::deserialize::<u64>(&self.db.get("latesttxid".to_string()).unwrap().unwrap()).unwrap();

        for txin in utxo {
            inamout += txin.get_value();
        }
        if inamout < amount {
            println!("Not enough money");
            return;
        }
        let mut txout = TxOut::new();
        txout.set_value(amount);
        txout.set_pubkey(to.clone());
        transaction.add_output(txout);

        let mut txout = TxOut::new();
        txout.set_pubkey(from.clone());
        txout.set_value(inamout-amount);
        transaction.add_output(txout);

        let mut block = Block::new("Transaction".to_string(), bincode::deserialize::<Block>(&self.db.get("latest").unwrap().unwrap()).unwrap().get_hash());
        
        block.add_transaction(transaction);
        let txid = bincode::deserialize::<u64>(&self.db.get("latesttxid".to_string()).unwrap().unwrap()).unwrap();
        block.mine_block(self.difficulty, from.clone(),txid);

        self.db.insert(block.get_hash(), bincode::serialize(&block).unwrap()).unwrap();
        self.db.insert("latest".to_string(), bincode::serialize(&block).unwrap()).unwrap();
        self.db.insert("latesttxid".to_string(), bincode::serialize(&(txid+block.get_transactions().len() as u64)).unwrap()).unwrap();
        self.utxo.update(block);

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
    pub fn add_block(&mut self, data: String, pubkey: String) {
        let test = self.db.get("latest").unwrap().unwrap();
        let test: Result<Block, Box<bincode::ErrorKind>> = bincode::deserialize::<Block>(&test);
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
        let txid = bincode::deserialize::<u64>(&self.db.get("latesttxid".to_string()).unwrap().unwrap()).unwrap();
        
        block.mine_block(difficulty, pubkey.clone(),txid);
        
        self.db.insert(block.get_hash(), bincode::serialize(&block).unwrap()).unwrap();
        self.db.insert("latest".to_string(), bincode::serialize(&block).unwrap()).unwrap();
        self.db.insert("latesttxid".to_string(), bincode::serialize(&(txid+block.get_transactions().len() as u64)).unwrap()).unwrap();
        self.utxo.update(block);
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

#[derive(Debug, Clone)]
pub struct UtxoSet {
    db: sled::Db,
}

impl UtxoSet {
    pub fn new() -> Self {
        let db = sled::open("utxo").unwrap();
        Self {
            db,
        }
    }


    pub fn find_spendable(&self, pubkey: String) -> Vec<Txin> {
        let mut utxo: Vec<Txin> = Vec::new();
        for kv in self.db.iter() {
            let (key, value) = kv.unwrap();
            let transaction: Transaction = bincode::deserialize(&value).unwrap();
            for txout in transaction.get_txouts() {
                if txout.pubkey == pubkey { // TODO: check the signature 
                    let mut txin = Txin::new();
                    txin.set_value(txout.get_value());
                    txin.set_pubkey(txout.pubkey);
                    utxo.push(txin);
                }
            }
        }
        utxo
    }

    pub fn update(&mut self, block: Block) {
        for transaction in block.get_transactions() {
            print!("{:?}",transaction.get_txid());
            self.db.insert(transaction.get_txid().to_string(), bincode::serialize(&transaction).unwrap()).unwrap();
            for txin in transaction.get_txins() {
                if txin.get_txid() == 0 {
                    continue;
                }
                let mut citetransaction = bincode::deserialize::<Transaction>(&self.db.get(txin.get_txid().to_string()).unwrap().unwrap()).unwrap();
                let mut newtxoutvec = Vec::<TxOut>::new();
                for txout in citetransaction.get_txouts() {
                    if txout.get_pubkey() != txin.get_pubkey() {
                        newtxoutvec.push(txout);
                    }
                }
                citetransaction.set_txouts(newtxoutvec);
                self.db.insert(citetransaction.get_txid().to_string(), bincode::serialize(&citetransaction).unwrap()).unwrap();
            }

        }
    }

}