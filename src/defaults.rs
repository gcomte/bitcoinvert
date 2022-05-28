use crate::currencies::{BitcoinUnit, Currencies};
use crate::Currency;
use home_config::HomeConfig;
use serde::{Deserialize, Serialize};
use std::error::Error;

const DEFAULTS_FILE: &str = "defaults.yaml";

pub struct Defaults {
    input_currency: Box<dyn Currency>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultsSerialized {
    input_currency: String,
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
        DefaultsSerialized {
            input_currency: BitcoinUnit::Sat.to_string(),
        }
    }
}

impl From<Defaults> for DefaultsSerialized {
    fn from(defaults: Defaults) -> Self {
        DefaultsSerialized {
            input_currency: defaults.input_currency.to_string(),
        }
    }
}

impl From<DefaultsSerialized> for Defaults {
    fn from(ds: DefaultsSerialized) -> Self {
        Defaults {
            input_currency: Currencies::parse_resort_to_default(&ds.input_currency),
        }
    }
}
