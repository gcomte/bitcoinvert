pub mod amount;
pub mod cli_input;
pub mod currencies;
pub mod currency;
pub mod defaults;
pub mod fiat_rates;
mod print;

use std::process;

use colored::*;

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

    if cli_input.output_currencies.len() == 1 {
        print::single_line(
            &output_values[0],
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

        print::multi_line(&output_values, &cli_input.output_currencies);
    }
}
