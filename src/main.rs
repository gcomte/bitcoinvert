mod blockchain_info_consumer;
pub mod cli_input;
pub mod currencies;
pub mod currency;
pub mod defaults;
pub mod exchange_rate_provider;

use crate::cli_input::CliInput;
use crate::currency::Currency;

fn main() {
    env_logger::init();

    let cli_input = CliInput::parse();

    println!("input currency: {}", cli_input.amount);
    println!("input currency: {:?}", cli_input.input_currency.to_string());

    // first: let's get the amount of sats.

    // each currency has an amount of sats it's worth.

    // println!("{:?}: {}", Fiat::CHF, x.btc_value(&Fiat::CHF));

    // println!(
    //     "output currencies: {:?}",
    //     cli_input
    //         .output_currencies
    //         .iter()
    //         .map(|c| format!("{}: {}", &c, &c.btc_value()))
    //         .collect::<Vec<String>>()
    // );

    let value_in_btc = cli_input.amount * cli_input.input_currency.btc_value();

    for currency in cli_input.output_currencies {
        println!("{}: {}", currency, value_in_btc / currency.btc_value());
    }
}
