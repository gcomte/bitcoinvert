use crate::currencies::fiat::Fiat;
use std::collections::HashMap;
use std::future::Future;

pub trait ExchangeRateApiConsumer {
    type Output: Future<Output = HashMap<Fiat, f64>>;

    fn fetch_api(&self) -> Self::Output;
}

pub struct ExchangeRateProvider<T: ExchangeRateApiConsumer> {
    pub data_source: T,
    pub data: Option<HashMap<Fiat, f64>>,
}

impl<T: ExchangeRateApiConsumer> ExchangeRateProvider<T> {
    pub fn btc_value(&self, currency: &Fiat) -> f64 {
        1.0 / self.data.as_ref().unwrap().get(currency).unwrap()
    }

    pub async fn fetch(&mut self) {
        if self.data.is_none() {
            self.data = Some(self.data_source.fetch_api().await);
        }
    }
}
