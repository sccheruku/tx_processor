extern crate csv;

use std::env;
use lib::transaction::{Transaction};
use lib::transaction_processor::TransactionProcessor;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.last().unwrap();

    // load the file
    let mut reader = csv::ReaderBuilder::new()
    .flexible(true)
    .from_path(filename)
    .unwrap();

    let mut processor = TransactionProcessor::new();

    for result in reader.records() {
        let record = result.unwrap();
        let transaction = Transaction::from(record);
        if transaction.is_valid() {
            processor.process_transaction(transaction);
        }
    }
    println!("client, available, held, total, locked");
    for account in processor.get_account_states() {
        println!("{}, {:.4}, {:.4}, {:.4}, {}", account.client, account.available, account.held, account.total, account.locked);
    }
}
