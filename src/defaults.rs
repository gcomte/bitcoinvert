use crate::currencies::{BitcoinUnit, Currencies, Fiat};
use crate::Currency;
use home_config::HomeConfig;
use serde::{Deserialize, Serialize};
use std::error::Error;

const DEFAULTS_FILE: &str = "defaults.yaml";

pub struct Defaults {
    input_currency: Box<dyn Currency>,
    output_currencies: Vec<Box<dyn Currency>>,
}

// Todo: Do proper serialization instead of a dedicated struct
#[derive(Serialize, Deserialize, Debug)]
struct DefaultsSerialized {
    input_currency: String,
    output_currencies: Vec<String>,
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
                log::error!(
                    "Can't load default values from file {}. Error: {}",
                    DEFAULTS_FILE,
                    err
                );
                panic!("{}", err);
            }
        }
    }

    fn load_defaults(config: &HomeConfig) -> Result<Defaults, Box<dyn Error>> {
        let defaults: DefaultsSerialized = serde_yaml::from_str(&config.read_to_string()?)?;
        log::debug!(
            "Reading contents of file {} : {:?}",
            config.path().display(),
            defaults
        );

        if Currencies::parse(&defaults.input_currency).is_err() {
            log::warn!(
                "Invalid input_currency defined in file {} currency: '{}'",
                config.path().display(),
                defaults.input_currency
            );
        }

        Ok(defaults.into())
    }

    fn setup(config: &HomeConfig) {
        config.save_yaml(Self::load_defaults_template()).unwrap();
    }

    fn load_defaults_template() -> DefaultsSerialized {
        Defaults {
            input_currency: Box::new(BitcoinUnit::Sat),
            output_currencies: vec![
                Box::new(BitcoinUnit::Btc),
                Box::new(BitcoinUnit::Sat),
                Box::new(BitcoinUnit::Msat),
                Box::new(Fiat::Usd),
                Box::new(Fiat::Eur),
                Box::new(Fiat::Gbp),
            ],
        }
        .into()
    }
}

impl From<Defaults> for DefaultsSerialized {
    fn from(defaults: Defaults) -> Self {
        let output_currencies: Vec<String> = defaults
            .output_currencies
            .iter()
            .map(|c| c.to_string())
            .collect();

        DefaultsSerialized {
            input_currency: defaults.input_currency.to_string(),
            output_currencies,
        }
    }
}

impl From<DefaultsSerialized> for Defaults {
    fn from(ds: DefaultsSerialized) -> Self {
        Defaults {
            input_currency: Currencies::parse_resort_to_default(&ds.input_currency),
            output_currencies: ds
                .output_currencies
                .iter()
                .map(|c| {
                    Currencies::parse(c).unwrap_or_else(|_| {
                        panic!(
                            "Output currency '{}' is not a valid currency! Revise your {}",
                            c, DEFAULTS_FILE
                        );
                    })
                })
                .collect(),
        }
    }
}
