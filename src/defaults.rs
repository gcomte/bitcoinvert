use home_config::HomeConfig;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::amount::Amount;
use crate::currency::btc::BitcoinUnit;
use crate::currency::fiat::Fiat;
use crate::Currency;

const DEFAULTS_FILE: &str = "defaults.yaml";

#[derive(Serialize, Deserialize)]
pub struct Defaults {
    amount: Amount,
    input_currency: Box<dyn Currency>,
    output_currencies: Vec<Box<dyn Currency>>,
}

impl Defaults {
    pub fn get_default_amount() -> Result<Amount, Box<dyn Error>> {
        Ok(Self::retrieve()?.amount)
    }

    pub fn get_default_input_currency() -> Result<Box<dyn Currency>, Box<dyn Error>> {
        Ok(Self::retrieve()?.input_currency)
    }

    pub fn get_default_output_currencies() -> Result<Vec<Box<dyn Currency>>, Box<dyn Error>> {
        Ok(Self::retrieve()?.output_currencies)
    }

    pub fn retrieve() -> Result<Defaults, Box<dyn Error>> {
        let config = HomeConfig::with_config_dir(env!("CARGO_PKG_NAME"), DEFAULTS_FILE);

        if !config.path().exists() {
            log::debug!(
                "{} does not exist. Creating it with template values.",
                config.path().display()
            );
            Self::setup(&config)?;
        }

        Self::load_defaults(&config).map_err(|err| {
            format!(
                "Can't load default values from file {}. Error: {}",
                DEFAULTS_FILE, err
            )
            .into()
        })
    }

    fn load_defaults(config: &HomeConfig) -> Result<Defaults, Box<dyn Error>> {
        let defaults = Self::parse_defaults(&config.read_to_string()?)?;
        log::debug!(
            "Reading contents of file {} --> input amount: {}, input currency: {}, output currencies: [{}]",
            config.path().display(),
            defaults.amount,
            defaults.input_currency,
            defaults
                .output_currencies
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        Ok(defaults)
    }

    fn parse_defaults(source: &str) -> Result<Defaults, Box<dyn Error>> {
        // Preserve legacy unquoted decimal amounts as their original lexemes.
        // Normal YAML scalar resolution would convert them through f64 first.
        let config = noyalib::ParserConfig::new().no_schema(true);
        Ok(noyalib::from_str_with_config(source, &config)?)
    }

    fn setup(config: &HomeConfig) -> Result<(), Box<dyn Error>> {
        config
            .save_yaml(Self::load_defaults_template())
            .map_err(|e| format!("Failed to save default config: {e:?}"))?;
        Ok(())
    }

    fn load_defaults_template() -> Defaults {
        Defaults {
            amount: Amount::from_integer(100_000_000),
            input_currency: Box::new(BitcoinUnit::SAT),
            output_currencies: vec![
                Box::new(BitcoinUnit::BTC),
                Box::new(BitcoinUnit::SAT),
                Box::new(BitcoinUnit::MSAT),
                Box::new(Fiat::USD),
                Box::new(Fiat::EUR),
                Box::new(Fiat::GBP),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_numeric_and_quoted_defaults_remain_exact() {
        for input in [
            "9007199254740993.001",
            "\"9007199254740993.001\"",
            "9007199254740993001e-3",
        ] {
            let yaml = format!("amount: {input}\ninput_currency:\n  BitcoinUnit: SAT\noutput_currencies:\n  - BitcoinUnit: BTC\n");
            let defaults = Defaults::parse_defaults(&yaml).unwrap();
            assert_eq!(defaults.amount.to_string(), "9007199254740993.001");
            assert_eq!(defaults.input_currency.to_string(), "SAT");
        }
    }

    #[test]
    fn rejects_invalid_configured_amounts() {
        for input in [".nan", ".inf", "1e309", "1e-309", "null", "false"] {
            let yaml = format!("amount: {input}\ninput_currency:\n  BitcoinUnit: SAT\noutput_currencies:\n  - BitcoinUnit: BTC\n");
            assert!(Defaults::parse_defaults(&yaml).is_err(), "{input}");
        }
    }

    #[test]
    fn template_roundtrip_preserves_configuration() {
        let original = Defaults::load_defaults_template();
        let yaml = noyalib::to_string(&original).unwrap();
        let restored = Defaults::parse_defaults(&yaml).unwrap();
        assert_eq!(restored.amount, original.amount);
        assert_eq!(
            restored.input_currency.to_string(),
            original.input_currency.to_string()
        );
        assert_eq!(
            restored.output_currencies.len(),
            original.output_currencies.len()
        );
    }
}
