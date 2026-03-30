use home_config::HomeConfig;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::currency::btc::BitcoinUnit;
use crate::currency::fiat::Fiat;
use crate::Currency;

const DEFAULTS_FILE: &str = "defaults.yaml";

#[derive(Serialize, Deserialize)]
pub struct Defaults {
    amount: f64,
    input_currency: Box<dyn Currency>,
    output_currencies: Vec<Box<dyn Currency>>,
}

impl Defaults {
    pub fn get_default_amount() -> Result<f64, Box<dyn Error>> {
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
        let defaults: Defaults = serde_yml::from_str(&config.read_to_string()?)?;
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

    fn setup(config: &HomeConfig) -> Result<(), Box<dyn Error>> {
        config
            .save_yaml(Self::load_defaults_template())
            .map_err(|e| format!("Failed to save default config: {e:?}"))?;
        Ok(())
    }

    fn load_defaults_template() -> Defaults {
        Defaults {
            amount: 100_000_000.0,
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
