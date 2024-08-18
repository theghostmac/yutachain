mod blockchain;

fn main() {
    let mut blockchain = blockchain::blocks::Blockchain::new();

    let first_transaction_data = "Alice -> Bob, 5 ETH".to_string();
    let second_transaction_data = "Jay -> Sharon, 10 ETH".to_string();

    blockchain.add_block(first_transaction_data);
    blockchain.add_block(second_transaction_data);

    for block in blockchain.blocks {
        println!("Block: {:#?}", block);
    }
}
