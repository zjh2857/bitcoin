use crypto::ed25519::keypair;
pub struct wallet {
    pub key: [u8;64],
    pub address: [u8;32],
    pub balance: u64,
}

impl wallet {
    pub fn new() -> wallet {
        let rand = rand::random::<[u8; 32]>();
        let (key, address) = keypair(&rand);
        wallet {
            key: key,
            address: address,
            balance: 0,
        }
    }
    pub fn get_address(&self) -> String {
        hex::encode(self.address)
    }
    pub fn get_key(&self) -> String {
        hex::encode(self.key)
    }
}


