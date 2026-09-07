use crate::amount::Amount;
use crate::currency::fiat::Fiat;
use crate::currency::ConversionError;
use std::collections::HashMap;

pub trait ExchangeRateApiConsumer {
    fn fetch_api(&self) -> Result<HashMap<Fiat, Amount>, ConversionError>;
}

pub struct ExchangeRateProvider<T: ExchangeRateApiConsumer> {
    pub data_source: T,
    pub data: Option<HashMap<Fiat, Amount>>,
}

impl<T: ExchangeRateApiConsumer> ExchangeRateProvider<T> {
    pub fn units_per_btc(&mut self, currency: &Fiat) -> Result<Amount, ConversionError> {
        if self.data.is_none() {
            self.data = Some(self.data_source.fetch_api()?);
        }
        let rate = self
            .data
            .as_ref()
            .and_then(|data| data.get(currency))
            .ok_or_else(|| ConversionError::MissingRate(currency.to_string()))?;
        if !rate.is_positive() {
            return Err(ConversionError::InvalidRate(currency.to_string()));
        }
        Ok(rate.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct MockApiConsumer {
        fetch_count: Cell<usize>,
    }

    impl ExchangeRateApiConsumer for MockApiConsumer {
        fn fetch_api(&self) -> Result<HashMap<Fiat, Amount>, ConversionError> {
            self.fetch_count.set(self.fetch_count.get() + 1);
            Ok(HashMap::from([
                (Fiat::USD, "50000.005".parse().unwrap()),
                (Fiat::EUR, "45000".parse().unwrap()),
                (Fiat::JPY, "7500000".parse().unwrap()),
            ]))
        }
    }

    fn mock_provider(data: Option<HashMap<Fiat, Amount>>) -> ExchangeRateProvider<MockApiConsumer> {
        ExchangeRateProvider {
            data_source: MockApiConsumer {
                fetch_count: Cell::new(0),
            },
            data,
        }
    }

    #[test]
    fn source_rates_are_preserved_exactly() {
        let mut provider = mock_provider(None);
        assert_eq!(
            provider.units_per_btc(&Fiat::USD).unwrap().to_string(),
            "50000.005"
        );
        assert_eq!(
            provider.units_per_btc(&Fiat::EUR).unwrap().to_string(),
            "45000"
        );
        assert_eq!(
            provider.units_per_btc(&Fiat::JPY).unwrap().to_string(),
            "7500000"
        );
    }

    #[test]
    fn data_is_cached_after_first_fetch() {
        let mut provider = mock_provider(None);
        assert!(provider.data.is_none());
        provider.units_per_btc(&Fiat::USD).unwrap();
        assert!(provider.data.is_some());
        assert_eq!(provider.data_source.fetch_count.get(), 1);
        provider.units_per_btc(&Fiat::EUR).unwrap();
        assert_eq!(provider.data_source.fetch_count.get(), 1);
    }

    #[test]
    fn missing_currency_returns_an_error() {
        let mut provider = mock_provider(Some(HashMap::new()));
        assert!(matches!(
            provider.units_per_btc(&Fiat::USD),
            Err(ConversionError::MissingRate(_))
        ));
    }

    #[test]
    fn zero_and_negative_rates_return_errors() {
        for rate in ["0", "-0.1"] {
            let mut provider =
                mock_provider(Some(HashMap::from([(Fiat::USD, rate.parse().unwrap())])));
            assert!(matches!(
                provider.units_per_btc(&Fiat::USD),
                Err(ConversionError::InvalidRate(_))
            ));
        }
    }
}
