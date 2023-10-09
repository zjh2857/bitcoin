
use crate::blockchain::Blockchain;
use clap::{App, Arg};
use crate::wallet;

pub struct Cil {
    pub blockchain: Blockchain,
}

impl Cil {
    pub fn new() -> Self {
        let blockchain = Blockchain::new();
        Self {
            blockchain,
        }
    }
    pub fn run(&mut self) {
        let matches = App::new("blockchain")
            .version("0.1.0")
            .author("zjh2857")
            .about("A simple blockchain written in Rust")
            .subcommand(App::new("block")
                .about("Add a new block to the blockchain")
                .arg(Arg::with_name("add")
                    .short('a')
                    .long("add")
                    .value_name("DATA")
                    .help("Add a new block to the blockchain")
                    .takes_value(true))
                .arg(Arg::with_name("public")
                    .short('p')
                    .long("public")
                    .value_name("PUBLIC")
                    .help("The public key of the miner")
                    .takes_value(true)))
            .subcommand(App::new("transaction")
                .about("Add a new transaction to the blockchain")
                .arg(Arg::with_name("from")
                    .short('f')
                    .long("from")
                    .value_name("FROM")
                    .help("The sender of the transaction")
                    .takes_value(true))
                .arg(Arg::with_name("to")
                    .short('o')
                    .long("to")
                    .value_name("TO")
                    .help("The receiver of the transaction")
                    .takes_value(true))
                .arg(Arg::with_name("amount")
                    .short('m')
                    .long("amount")
                    .value_name("AMOUNT")
                    .help("The amount of the transaction")
                    .takes_value(true)))
            .subcommand(App::new("wallet")
                .about("Add a new wallet")
                .arg(Arg::with_name("generate")
                    .short('g')
                    .long("generate")
                    .help("Generate a new wallet")))
            .arg(Arg::with_name("add")
                .short('a')
                .long("add")
                .value_name("DATA")
                .help("Add a new block to the blockchain")
                .takes_value(true))
            .arg(Arg::with_name("list")
                .short('l')
                .long("list")
                .help("List all blocks in the blockchain"))
            .arg(Arg::with_name("validate")
                .short('v')
                .long("validate")
                .help("Validate the blockchain"))
            .arg(Arg::with_name("difficulty")
                .short('d')
                .long("difficulty")
                .value_name("DIFFICULTY")
                .help("Set the difficulty of mining")
                .takes_value(true)
                .default_value("1"))
            .arg(Arg::with_name("transaction")
                .short('t')
                .long("transaction")
                .value_name("TRANSACTION")
                .help("Add a new transaction to the blockchain")
                .takes_value(true))
            .arg(Arg::with_name("from")
                .short('f')
                .long("from")
                .value_name("FROM")
                .help("The sender of the transaction")
                .takes_value(true))
            .arg(Arg::with_name("to")
                .short('o')
                .long("to")
                .value_name("TO")
                .help("The receiver of the transaction")
                .takes_value(true))
            .arg(Arg::with_name("amount")
                .short('m')
                .long("amount")
                .value_name("AMOUNT")
                .help("The amount of the transaction")
                .takes_value(true))
            .get_matches();
        if matches.is_present("list") {
            log::info!("list");
            self.blockchain.show();
        } else if matches.is_present("validate") {
            self.blockchain.is_valid();
        } else if matches.is_present("difficulty") {
            let difficulty = matches.value_of("difficulty").unwrap();
            let difficulty: u32 = difficulty.parse().unwrap();
            self.blockchain.set_difficulty(difficulty);
        } 
        if let Some(matches) = matches.subcommand_matches("transaction") {
            let from = matches.value_of("from").unwrap();
            let to = matches.value_of("to").unwrap();
            let amount = matches.value_of("amount").unwrap();
            self.blockchain.add_transaction(String::from(from), String::from(to),String::from(from), amount.parse().unwrap());
        }
        if let Some(matches) = matches.subcommand_matches("wallet") {
            if matches.is_present("generate") {
                let wallet = wallet::wallet::new();
                println!("Public key: {}", wallet.get_address());
                println!("Private key: {}", wallet.get_key());
            }
        }
        if let Some(matches) = matches.subcommand_matches("block") {
            let data = matches.value_of("add").unwrap();
            let address = matches.value_of("public").unwrap();
            log::info!("AAAAAAAAA");

            self.blockchain.add_block(String::from(data), String::from(address));
        }
    }
}
