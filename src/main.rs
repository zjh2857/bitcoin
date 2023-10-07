// 
use btc::cil::Cil;

#[macro_use]
use log::{info};

use env_logger::Env;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    info!("[bar] info");
    let mut app = Cil::new();
    app.run();
    Ok(())
}

//test
#[cfg(test)]
mod tests {
    use btc::blockchain::Blockchain;
    #[test]
    fn test_blockchain() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("Send 1 BTC to Ivan"));
        blockchain.add_block(String::from("Send 2 more BTC to Ivan"));
        assert!(blockchain.is_valid());
    }
}
