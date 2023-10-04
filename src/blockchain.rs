use crate::block::Block;
#[derive(Debug, Clone)]
pub struct Blockchain {
    difficulty: u32,
    db : sled::Db,
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
        }
    }
    
    pub fn add_block(&mut self, data: String) {
        // let previous_hash = bincode::deserialize(&self.db.get("latest").unwrap().unwrap()).unwrap();
        let test = self.db.get("latest").unwrap().unwrap();
        let test = bincode::deserialize::<Block>(&test);
        match test {
            
            Ok(_) => {
                println!("Blockchain already exists114514.");
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

