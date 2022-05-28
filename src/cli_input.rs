use crate::currencies::Currencies;
use crate::defaults::Defaults;
use crate::Currency;
use std::error::Error;
use std::num::ParseFloatError;
use std::{fmt, process};

pub struct CliInput {
    pub amount: f64,
    pub input_currency: Box<dyn Currency>,
    pub output_currency: Option<Box<dyn Currency>>,
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

impl CliInput {
    pub fn parse() -> Result<Self, InputError> {
        let amount = Self::parse_amount()?;
        let input_currency = Self::parse_input_currency();
        let output_currency = Self::parse_output_currency();

        let output_currence_debug_text = match &output_currency {
            Some(currency) => format!("'{}'", currency),
            None => "[undefined]".to_string(),
        };
        log::debug!(
            "Parsing CLI arguments and config > amount: {}, input-currency: '{}', output-currency: {}",
            amount,
            input_currency,
            output_currence_debug_text
        );

        Ok(Self {
            amount: amount.parse()?,
            input_currency,
            output_currency,
        })
    }

    fn parse_amount() -> Result<String, InputError> {
        match std::env::args().nth(1) {
            Some(amount) => Ok(amount),
            None => Err(InputError::new("No amount provided!")),
        }
    }

    fn parse_input_currency() -> Box<dyn Currency> {
        match std::env::args().nth(2) {
            Some(currency) => match Currencies::parse(&currency) {
                Ok(currency) => currency,
                Err(_) => {
                    eprintln!("\"{}\" is not a valid currency!", currency);
                    process::exit(exitcode::USAGE);
                }
            },
            None => Defaults::get_default_currency(),
        }
    }

    fn parse_output_currency() -> Option<Box<dyn Currency>> {
        let output_currency = std::env::args().nth(3);

        match output_currency {
            Some(currency) => match Currencies::parse(&currency) {
                Ok(currency) => Some(currency),
                Err(_) => {
                    log::warn!("\"{}\" is not a valid currency! Continuing with multiple output currencies", currency);
                    None
                }
            },
            None => None,
        }
    }
}
