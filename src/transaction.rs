use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Txin {
    pub txid:u64,
    pub value: u64,
    pub signature: String,
    pub pubkey: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxOut {
    pub value: u64,
    pub pubkey: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: u64,
    pub version: u64,
    pub txins: Vec<Txin>,
    pub txouts: Vec<TxOut>,
    pub locktime: u32,
}

impl Txin {
    pub fn new() -> Txin {
        Txin {
            txid: 0,
            value: 0,
            signature: String::from(""),
            pubkey: String::from(""),
        }
    }
    
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    pub fn get_signature(&self) -> String {
        self.signature.clone()
    }

    pub fn get_txid(&self) -> u64 {
        self.txid
    }

    pub fn set_txid(&mut self, txid: u64) {
        self.txid = txid;
    }
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
    pub fn get_value(&self) -> u64 {
        self.value
    }
    pub fn set_pubkey(&mut self, script_sig: String) {
        self.pubkey = script_sig;
    }

    pub fn get_pubkey(&self) -> String {
        self.pubkey.clone()
    }
}


impl TxOut {
    pub fn new() -> TxOut {
        TxOut {
            value: 0,
            pubkey: String::from(""),
        }
    }
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }

    pub fn set_pubkey(&mut self, pubkey: String) {
        self.pubkey = pubkey;
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    pub fn get_pubkey(&self) -> String {
        self.pubkey.clone()
    }

}
impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            txid: 0,
            version: 1,
            txins: Vec::new(),
            txouts: Vec::new(),
            locktime: 0,
        }
    }
    pub fn add_input(&mut self, txin: Txin) {
        self.txins.push(txin);
    }

    pub fn add_output(&mut self, txout: TxOut) {
        self.txouts.push(txout);
    }

    pub fn get_txins(&self) -> Vec<Txin> {
        self.txins.clone()
    }

    pub fn get_txouts(&self) -> Vec<TxOut> {
        self.txouts.clone()
    }
    pub fn get_locktime(&self) -> u32 {
        self.locktime
    }
    pub fn set_txouts(&mut self, txouts: Vec<TxOut>) {
        self.txouts = txouts;
    }
    pub fn set_txid(&mut self, txid: u64) {
        self.txid = txid;
    }
    pub fn get_txid(&self) -> u64 {
        self.txid
    }
}
