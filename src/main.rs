pub mod amount;
pub mod cli_input;
pub mod currencies;
pub mod currency;
pub mod defaults;
pub mod fiat_rates;
mod print;

use std::process;

use crate::cli_input::CliInput;
use crate::currency::Currency;

fn main() {
    env_logger::init();

    let cli_input = match CliInput::parse() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("{e}");
            process::exit(exitcode::USAGE);
        }
    };

    let output_values = match currency::convert(
        &cli_input.amount,
        &*cli_input.input_currency,
        &cli_input.output_currencies,
        cli_input.integer,
    ) {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            process::exit(exitcode::DATAERR);
        }
    };

    let input = print::Money {
        amount: cli_input.amount.to_string(),
        currency: cli_input.input_currency.to_string(),
    };
    let outputs: Vec<_> = cli_input
        .output_currencies
        .iter()
        .zip(output_values)
        .map(|(currency, amount)| print::Money {
            amount: amount.to_string(),
            currency: currency.to_string(),
        })
        .collect();

    if cli_input.json {
        if let Err(error) = print::json(&input, &outputs) {
            eprintln!("Failed to format JSON output: {error}");
            process::exit(exitcode::SOFTWARE);
        }
    } else if let [output] = outputs.as_slice() {
        print::single_line(output, cli_input.clean);
    } else {
        print::multi_line(&input, &outputs);
    }
}
