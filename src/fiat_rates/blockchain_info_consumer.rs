use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

use crate::amount::Amount;
use crate::currency::fiat::Fiat;
use crate::currency::ConversionError;
use crate::fiat_rates::exchange_rate_provider::ExchangeRateApiConsumer;

const SOURCE_API: &str = "https://blockchain.info/ticker";
// Bound the response before parsing arbitrary-precision JSON numbers.
const MAX_TICKER_BYTES: u64 = 1024 * 1024;

pub struct ApiConsumer;

#[derive(Deserialize)]
struct Ticker {
    // arbitrary_precision preserves decimal and scientific JSON lexemes.
    last: serde_json::Number,
}

impl ApiConsumer {
    fn parse_data(source: &[u8]) -> Result<HashMap<Fiat, Amount>, ConversionError> {
        let tickers: HashMap<String, Ticker> = serde_json::from_slice(source).map_err(|error| {
            ConversionError::RateSource(format!("invalid ticker JSON: {error}"))
        })?;
        let mut rates = HashMap::new();
        for (name, ticker) in tickers {
            let Ok(currency) = Fiat::from_str(&name) else {
                continue;
            };
            let rate: Amount = ticker
                .last
                .to_string()
                .parse()
                .map_err(|_| ConversionError::InvalidRate(name.clone()))?;
            if !rate.is_positive() {
                return Err(ConversionError::InvalidRate(name));
            }
            rates.insert(currency, rate);
        }
        Ok(rates)
    }

    fn fetch_data() -> Result<HashMap<Fiat, Amount>, ConversionError> {
        log::debug!("Request exchange rate data from {}", SOURCE_API);
        let response = reqwest::blocking::get(SOURCE_API)
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(|error| ConversionError::RateSource(error.to_string()))?;
        log::debug!("Received response from {}", SOURCE_API);
        let mut source = Vec::new();
        response
            .take(MAX_TICKER_BYTES + 1)
            .read_to_end(&mut source)
            .map_err(|error| ConversionError::RateSource(error.to_string()))?;
        if source.len() as u64 > MAX_TICKER_BYTES {
            return Err(ConversionError::RateSource(
                "ticker response exceeds 1 MiB".to_string(),
            ));
        }
        Self::parse_data(&source)
    }
}

impl ExchangeRateApiConsumer for ApiConsumer {
    fn fetch_api(&self) -> Result<HashMap<Fiat, Amount>, ConversionError> {
        Self::fetch_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_call_must_not_fail() {
        ApiConsumer::fetch_data().unwrap();
    }

    #[test]
    fn json_rates_retain_every_decimal_digit() {
        for literal in ["9007199254740993.001", "9007199254740993001e-3"] {
            let data = format!("{{\"USD\":{{\"last\":{literal}}}}}");
            let rates = ApiConsumer::parse_data(data.as_bytes()).unwrap();
            assert_eq!(rates[&Fiat::USD].to_string(), "9007199254740993.001");
        }
    }

    #[test]
    fn invalid_remote_rates_are_rejected() {
        for literal in ["0", "-1", "1e309", "1e-309", "NaN", "null", "\"50000\""] {
            let data = format!("{{\"USD\":{{\"last\":{literal}}}}}");
            assert!(
                ApiConsumer::parse_data(data.as_bytes()).is_err(),
                "{literal}"
            );
        }
    }
}
