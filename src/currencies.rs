use std::str::FromStr;

use crate::currency::btc::BitcoinUnit;
use crate::currency::fiat::Fiat;
use crate::currency::Currency;

pub struct Currencies;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("CurrencyParseError: {msg}")]
pub struct CurrencyParseError {
    msg: String,
}

impl Currencies {
    pub fn parse(input: &str) -> Result<Box<dyn Currency>, CurrencyParseError> {
        if let Ok(btc) = BitcoinUnit::from_str(input) {
            return Ok(Box::new(btc));
        }

        if let Ok(fiat) = Fiat::from_str(input) {
            return Ok(Box::new(fiat));
        }

        Err(CurrencyParseError {
            msg: format!(
                "Unable to parse the following currency: {}",
                input.to_string()
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_fiat_currency() {
        let currency_lowercase = Currencies::parse("usd").unwrap();
        let currency_capitalized = Currencies::parse("Usd").unwrap();
        let currency_uppercase = Currencies::parse("USD").unwrap();

        assert_eq!(currency_lowercase.to_string(), "USD");
        assert_eq!(currency_capitalized.to_string(), "USD");
        assert_eq!(currency_uppercase.to_string(), "USD");
    }

    #[test]
    fn should_return_correct_bitcoin_denomination() {
        let currency_lowercase = Currencies::parse("btc").unwrap();
        let currency_capitalized = Currencies::parse("Btc").unwrap();
        let currency_uppercase = Currencies::parse("BTC").unwrap();

        assert_eq!(currency_lowercase.to_string(), "BTC");
        assert_eq!(currency_capitalized.to_string(), "BTC");
        assert_eq!(currency_uppercase.to_string(), "BTC");
    }

    #[test]
    fn incorrect_use_should_return_error() {
        let currency_empty_string = Currencies::parse("");

        assert!(currency_empty_string.is_err());
        assert!(matches!(
            currency_empty_string,
            Err(CurrencyParseError { msg: _ })
        ));

        let currency_non_existant = Currencies::parse("non-existant");

        assert!(currency_non_existant.is_err());
        assert!(matches!(
            currency_non_existant,
            Err(CurrencyParseError { msg: _ })
        ));
    }
}
