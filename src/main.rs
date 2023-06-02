pub mod cli_input;
pub mod currencies;
pub mod currency;
pub mod defaults;
pub mod fiat_rates;
mod print;

use colored::*;

use crate::cli_input::CliInput;
use crate::currency::Currency;

fn main() {
    env_logger::init();

    if 100 > i32::MAX {} // Nevermind. I'm just testing whether the GitHub clippy check works

    let cli_input = CliInput::parse();
    let value_in_btc = cli_input.amount * cli_input.input_currency.btc_value();

    if cli_input.output_currencies.len() == 1 {
        let mut output_value = value_in_btc / cli_input.output_currencies[0].btc_value();

        if cli_input.integer {
            output_value = output_value.round();
        }

        print::single_line(
            output_value,
            &*cli_input.output_currencies[0],
            cli_input.clean,
        );
    } else {
        if cli_input.clean {
            eprintln!(
                "\n{}\n",
                "Cannot use clean mode for multi currency output"
                    .to_string()
                    .yellow()
            );
        }

        print::multi_line(
            value_in_btc,
            &cli_input.output_currencies,
            cli_input.integer,
        );
    }
}
