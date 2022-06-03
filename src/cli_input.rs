use crate::currencies::Currencies;
use crate::defaults::Defaults;
use crate::Currency;
use clap::Parser;
use std::error::Error;
use std::num::ParseFloatError;
use std::{fmt, process};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The amount of money to convert
    pub amount: Option<f64>,
    /// The currency to convert from
    pub input_currency: Option<String>,
    /// The currency to convert to
    pub output_currency: Option<String>,
}

pub struct CliInput {
    pub amount: f64,
    pub input_currency: Box<dyn Currency>,
    pub output_currencies: Vec<Box<dyn Currency>>,
}

#[derive(Debug)]
pub struct InputError {
    details: String,
}

impl InputError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InputError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseFloatError> for InputError {
    fn from(err: ParseFloatError) -> Self {
        Self::new(&err.to_string())
    }
}

impl From<Args> for CliInput {
    fn from(args: Args) -> Self {
        Self {
            amount: Self::parse_amount(args.amount),
            input_currency: Self::parse_input_currency(&args.input_currency),
            output_currencies: Self::parse_output_currency(&args.output_currency),
        }
    }
}

impl CliInput {
    pub fn parse() -> Self {
        Args::parse().into()
    }

    fn parse_amount(input: Option<f64>) -> f64 {
        match input {
            Some(amount) => amount,
            None => Defaults::get_default_amount(),
        }
    }

    fn parse_input_currency(string: &Option<String>) -> Box<dyn Currency> {
        match string {
            Some(currency) => match Currencies::parse(currency) {
                Ok(currency) => currency,
                Err(_) => {
                    eprintln!("\"{}\" is not a valid currency!", currency);
                    process::exit(exitcode::USAGE);
                }
            },
            None => Defaults::get_default_input_currency(),
        }
    }

    fn parse_output_currency(string: &Option<String>) -> Vec<Box<dyn Currency>> {
        if let Some(string) = string {
            match Currencies::parse(string) {
                Ok(currency) => return vec![currency],
                Err(_) => {
                    log::warn!("\"{}\" is not a valid currency! Continuing with multiple output currencies", string);
                }
            }
        }

        Defaults::get_default_output_currencies()
    }
}
