use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Error};
use std::str::FromStr;
use strum_macros::EnumString;

#[typetag::serde(tag = "currency-type", content = "unit")]
pub trait Currency: Display {}

pub struct Currencies;

impl Currencies {
    pub fn parse(input: &str) -> Result<Box<dyn Currency>, Error> {
        if let Ok(btc) = BitcoinUnit::from_str(input) {
            return Ok(Box::new(btc));
        }

        if let Ok(fiat) = Fiat::from_str(input) {
            return Ok(Box::new(fiat));
        }

        Err(Default::default())
    }

    pub fn parse_resort_to_default(input: &str) -> Box<dyn Currency> {
        if let Ok(currency) = Self::parse(input) {
            return currency;
        }

        let default = Box::new(BitcoinUnit::Sat);
        log::warn!("Resort to default currency: {}", default);

        default
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum BitcoinUnit {
    Btc,  // bitcoin
    Mbtc, // milli-bitcoin
    Bits, // μBTC, micro-bitcoin
    Sat,  // satoshi
    Msat, // milli-satoshi
}

impl Display for BitcoinUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[typetag::serde]
impl Currency for BitcoinUnit {}

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Fiat {
    Ars,
    Aud,
    Brl,
    Cad,
    Chf,
    Clp,
    Cny,
    Czk,
    Dkk,
    Eur,
    Gbp,
    Hkd,
    Hrk,
    Huf,
    Inr,
    Isk,
    Jpy,
    Krw,
    Nzd,
    Pln,
    Ron,
    Rub,
    Sek,
    Sgd,
    Thb,
    Try,
    Twd,
    Usd,
}

impl Display for Fiat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[typetag::serde]
impl Currency for Fiat {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_fiat_currency() {
        let currency_lowercase_default = Currencies::parse_resort_to_default("usd");
        let currency_capitalized_default = Currencies::parse_resort_to_default("Usd");
        let currency_uppercase_default = Currencies::parse_resort_to_default("USD");

        let currency_lowercase = Currencies::parse("usd").unwrap();
        let currency_capitalized = Currencies::parse("Usd").unwrap();
        let currency_uppercase = Currencies::parse("USD").unwrap();

        assert_eq!(currency_lowercase_default.to_string(), "Usd");
        assert_eq!(currency_capitalized_default.to_string(), "Usd");
        assert_eq!(currency_uppercase_default.to_string(), "Usd");

        assert_eq!(currency_lowercase.to_string(), "Usd");
        assert_eq!(currency_capitalized.to_string(), "Usd");
        assert_eq!(currency_uppercase.to_string(), "Usd");
    }

    #[test]
    fn should_return_correct_bitcoin_denomination() {
        let currency_lowercase_default = Currencies::parse_resort_to_default("btc");
        let currency_capitalized_default = Currencies::parse_resort_to_default("Btc");
        let currency_uppercase_default = Currencies::parse_resort_to_default("BTC");

        let currency_lowercase = Currencies::parse("btc").unwrap();
        let currency_capitalized = Currencies::parse("Btc").unwrap();
        let currency_uppercase = Currencies::parse("BTC").unwrap();

        assert_eq!(currency_lowercase_default.to_string(), "Btc");
        assert_eq!(currency_capitalized_default.to_string(), "Btc");
        assert_eq!(currency_uppercase_default.to_string(), "Btc");

        assert_eq!(currency_lowercase.to_string(), "Btc");
        assert_eq!(currency_capitalized.to_string(), "Btc");
        assert_eq!(currency_uppercase.to_string(), "Btc");
    }

    #[test]
    fn incorrect_use_should_return_error() {
        let currency_empty_string = Currencies::parse("");
        let currency_non_existant = Currencies::parse("non-existant");

        assert!(currency_empty_string.is_err());
        assert!(currency_non_existant.is_err());
    }

    #[test]
    fn incorrect_use_should_default_to_bitcoin_sat() {
        let currency_empty_string = Currencies::parse_resort_to_default("");
        let currency_non_existant = Currencies::parse_resort_to_default("non-existant");

        assert_eq!(currency_empty_string.to_string(), "Sat");
        assert_eq!(currency_non_existant.to_string(), "Sat");
    }
}
