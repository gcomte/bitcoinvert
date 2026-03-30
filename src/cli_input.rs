use clap::Parser;
use colored::*;
use regex::Regex;
use si_unit_prefix::SiUnitPrefix;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

use crate::currencies::Currencies;
use crate::defaults::Defaults;
use crate::Currency;

const THOUSAND_SEPARATOR_PATTERN: &str = r",|\s|'";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The amount of money to convert (SI units are supported => 1k = 1,000, 1M = 1,000,000, etc.)
    pub amount: Option<String>,
    /// The currency to convert from
    pub input_currency: Option<String>,
    /// The currency to convert to
    pub output_currency: Option<String>,
    #[arg(short, long, help = "Prints a clean number; no separators, no unit.")]
    clean: bool,
    #[arg(short, long, help = "Rounds the output to the nearest integer")]
    integer: bool,
}

pub struct CliInput {
    pub amount: f64,
    pub input_currency: Box<dyn Currency>,
    pub output_currencies: Vec<Box<dyn Currency>>,
    pub clean: bool,
    pub integer: bool,
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

impl TryFrom<Args> for CliInput {
    type Error = InputError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: Self::parse_amount(args.amount)?,
            input_currency: Self::parse_input_currency(&args.input_currency)?,
            output_currencies: Self::parse_output_currency(&args.output_currency)?,
            clean: args.clean,
            integer: args.integer,
        })
    }
}

impl CliInput {
    pub fn parse() -> Result<Self, InputError> {
        Args::parse().try_into()
    }

    fn parse_amount(input: Option<String>) -> Result<f64, InputError> {
        match input {
            Some(mut amount) => {
                // check whether last character is an SI unit
                let mut multiplier = 1.0;
                let last_char = amount.chars().last().unwrap();

                if let Some(si_prefix) = SiUnitPrefix::parse_from_str(&last_char.to_string()) {
                    multiplier = si_prefix.as_f64();

                    // remove last character
                    amount = amount[..amount.len() - 1].to_string();
                }

                match Self::strip_thousand_separators(&amount).parse::<f64>() {
                    Ok(amount) => Ok(amount * multiplier),
                    Err(_) => Err(InputError::new(&format!(
                        "\"{}\" is not a valid amount!",
                        amount
                    ))),
                }
            }
            None => Defaults::get_default_amount()
                .map_err(|e| InputError::new(&format!("Failed to load default amount: {e}"))),
        }
    }

    fn parse_input_currency(string: &Option<String>) -> Result<Box<dyn Currency>, InputError> {
        match string {
            Some(currency) => match Currencies::parse(currency) {
                Ok(currency) => Ok(currency),
                Err(_) => Err(InputError::new(&format!(
                    "\"{}\" is not a valid (input) currency!",
                    currency
                ))),
            },
            None => Defaults::get_default_input_currency().map_err(|e| {
                InputError::new(&format!("Failed to load default input currency: {e}"))
            }),
        }
    }

    fn parse_output_currency(
        string: &Option<String>,
    ) -> Result<Vec<Box<dyn Currency>>, InputError> {
        if let Some(string) = string {
            match Currencies::parse(string) {
                Ok(currency) => return Ok(vec![currency]),
                Err(_) => {
                    eprintln!("\n{}\n", format!("\"{}\" is not a valid (output) currency! Showing multiple output currencies instead.", string).yellow());
                }
            }
        }

        Defaults::get_default_output_currencies()
            .map_err(|e| InputError::new(&format!("Failed to load default output currencies: {e}")))
    }

    fn strip_thousand_separators(amount: &str) -> String {
        let re = Regex::new(THOUSAND_SEPARATOR_PATTERN).unwrap();
        re.replace_all(amount, "").to_string()
    }
}
