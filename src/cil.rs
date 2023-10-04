
use crate::blockchain::Blockchain;
use clap::{App, Arg};


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
            .get_matches();
        if matches.is_present("add") {
            let data = matches.value_of("add").unwrap();
            self.blockchain.add_block(String::from(data));
        } else if matches.is_present("list") {
            self.blockchain.show();
        } else if matches.is_present("validate") {
            self.blockchain.is_valid();
        }
    }
}
