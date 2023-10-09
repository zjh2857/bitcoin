rm -rf ./blockchain

cargo run -- -l
cargo run -- block -a test1 -p 1234
cargo run -- block -a test2 -p 1234
cargo run -- block -a test3 -p 1234
cargo run -- transaction -f 1234 -o 5678 -m 10 
cargo run -- transaction -f 1234 -o 5678 -m 2000
cargo run -- transaction -f 5678 -o 1234 -m 10
cargo run -- transaction -f 5678 -o 114514 -m 50

cargo run -- -l