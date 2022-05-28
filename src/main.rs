pub mod blockchain_info_consumer;
pub mod cli_input;
pub mod currencies;
pub mod defaults;

use crate::blockchain_info_consumer::ApiConsumer;
use crate::cli_input::CliInput;
use crate::currencies::Currency;
use std::process;

// todo general exchange rate provider, that could be replaced

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli_input = CliInput::parse().unwrap_or_else(|_| {
        eprintln!("Incorrect usage. Run 'bitcoinvert amount [from-currency] [to-currency]'");
        process::exit(exitcode::USAGE);
    });

    let currency = cli_input.input_currency;
    println!("currency: {:?}", currency.to_string());

    println!("{:?}", ApiConsumer::fetch_data().await);
}
