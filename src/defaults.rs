use crate::currencies::{BitcoinUnit, Fiat};
use crate::Currency;
use home_config::HomeConfig;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process;

const DEFAULTS_FILE: &str = "defaults.yaml";

#[derive(Serialize, Deserialize)]
pub struct Defaults {
    input_currency: Box<dyn Currency>,
    output_currencies: Vec<Box<dyn Currency>>,
}

impl Defaults {
    pub fn get_default_currency() -> Box<dyn Currency> {
        Self::retrieve().input_currency
    }

    pub fn retrieve() -> Defaults {
        let config = HomeConfig::new(env!("CARGO_PKG_NAME"), DEFAULTS_FILE);

        if !config.path().exists() {
            log::debug!(
                "{} does not exist. Creating it with template values.",
                config.path().display()
            );
            Self::setup(&config);
        }

        match Self::load_defaults(&config) {
            Ok(defaults) => defaults,
            Err(err) => {
                eprintln!(
                    "Can't load default values from file {}. Error: {}",
                    DEFAULTS_FILE, err
                );
                process::exit(exitcode::USAGE);
            }
        }
    }

    fn load_defaults(config: &HomeConfig) -> Result<Defaults, Box<dyn Error>> {
        let defaults: Defaults = serde_yaml::from_str(&config.read_to_string()?)?;
        log::debug!(
            "Reading contents of file {} --> input-currency: {}, output-currencies: [{}]",
            config.path().display(),
            defaults.input_currency.to_string(),
            defaults
                .output_currencies
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        Ok(defaults)
    }

    fn setup(config: &HomeConfig) {
        config.save_yaml(Self::load_defaults_template()).unwrap();
    }

    fn load_defaults_template() -> Defaults {
        Defaults {
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
