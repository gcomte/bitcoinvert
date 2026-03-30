use crate::currency::fiat::Fiat;
use std::collections::HashMap;

pub trait ExchangeRateApiConsumer {
    fn fetch_api(&self) -> HashMap<Fiat, f64>;
}

pub struct ExchangeRateProvider<T: ExchangeRateApiConsumer> {
    pub data_source: T,
    pub data: Option<HashMap<Fiat, f64>>,
}

impl<T: ExchangeRateApiConsumer> ExchangeRateProvider<T> {
    pub fn btc_value(&mut self, currency: &Fiat) -> f64 {
        self.fetch();

        1.0 / self.data.as_ref().unwrap().get(currency).unwrap()
    }

    fn fetch(&mut self) {
        if self.data.is_none() {
            self.data = Some(self.data_source.fetch_api());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockApiConsumer {
        fetch_count: &'static AtomicUsize,
    }

    impl ExchangeRateApiConsumer for MockApiConsumer {
        fn fetch_api(&self) -> HashMap<Fiat, f64> {
            self.fetch_count.fetch_add(1, Ordering::SeqCst);
            let mut rates = HashMap::new();
            rates.insert(Fiat::USD, 50_000.0);
            rates.insert(Fiat::EUR, 45_000.0);
            rates.insert(Fiat::JPY, 7_500_000.0);
            rates
        }
    }

    fn mock_provider_with_data(rates: HashMap<Fiat, f64>) -> ExchangeRateProvider<MockApiConsumer> {
        static UNUSED: AtomicUsize = AtomicUsize::new(0);
        ExchangeRateProvider {
            data_source: MockApiConsumer {
                fetch_count: &UNUSED,
            },
            data: Some(rates),
        }
    }

    fn mock_provider_with_fetch(
        counter: &'static AtomicUsize,
    ) -> ExchangeRateProvider<MockApiConsumer> {
        ExchangeRateProvider {
            data_source: MockApiConsumer {
                fetch_count: counter,
            },
            data: None,
        }
    }

    #[test]
    fn btc_value_returns_inverse_of_rate() {
        let mut rates = HashMap::new();
        rates.insert(Fiat::USD, 50_000.0);
        let mut provider = mock_provider_with_data(rates);

        let btc_value = provider.btc_value(&Fiat::USD);
        assert!((btc_value - 1.0 / 50_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn data_is_cached_after_first_fetch() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let mut provider = mock_provider_with_fetch(&COUNTER);

        assert!(provider.data.is_none());
        provider.btc_value(&Fiat::USD);
        assert!(provider.data.is_some());
        assert_eq!(COUNTER.load(Ordering::SeqCst), 1);

        // Second call uses cached data — fetch_api not called again
        provider.btc_value(&Fiat::EUR);
        assert_eq!(COUNTER.load(Ordering::SeqCst), 1);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_currency_panics() {
        let rates = HashMap::new();
        let mut provider = mock_provider_with_data(rates);
        provider.btc_value(&Fiat::USD);
    }

    #[test]
    fn zero_rate_produces_infinity() {
        let mut rates = HashMap::new();
        rates.insert(Fiat::USD, 0.0);
        let mut provider = mock_provider_with_data(rates);

        let btc_value = provider.btc_value(&Fiat::USD);
        assert!(btc_value.is_infinite());
    }

    #[test]
    fn multiple_currencies_return_correct_values() {
        let mut rates = HashMap::new();
        rates.insert(Fiat::USD, 50_000.0);
        rates.insert(Fiat::EUR, 45_000.0);
        rates.insert(Fiat::JPY, 7_500_000.0);
        let mut provider = mock_provider_with_data(rates);

        assert!((provider.btc_value(&Fiat::USD) - 1.0 / 50_000.0).abs() < f64::EPSILON);
        assert!((provider.btc_value(&Fiat::EUR) - 1.0 / 45_000.0).abs() < f64::EPSILON);
        assert!((provider.btc_value(&Fiat::JPY) - 1.0 / 7_500_000.0).abs() < f64::EPSILON);
    }
}
