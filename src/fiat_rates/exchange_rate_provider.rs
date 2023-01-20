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
