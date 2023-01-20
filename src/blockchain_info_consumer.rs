use crate::currency::fiat::Fiat;
use crate::exchange_rate_provider::ExchangeRateApiConsumer;
use serde::Deserialize;
use std::collections::HashMap;
use std::pin::Pin;

const SOURCE_API: &str = "https://blockchain.info/ticker";

pub struct ApiConsumer;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(rename(deserialize = "15m"))]
    avg: f64,
    last: f64,
    buy: f64,
    sell: f64,
    symbol: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Currencies {
    #[serde(rename(deserialize = "ARS"))]
    ars: Ticker,
    #[serde(rename(deserialize = "AUD"))]
    aud: Ticker,
    #[serde(rename(deserialize = "BRL"))]
    brl: Ticker,
    #[serde(rename(deserialize = "CAD"))]
    cad: Ticker,
    #[serde(rename(deserialize = "CHF"))]
    chf: Ticker,
    #[serde(rename(deserialize = "CLP"))]
    clp: Ticker,
    #[serde(rename(deserialize = "CNY"))]
    cny: Ticker,
    #[serde(rename(deserialize = "CZK"))]
    czk: Ticker,
    #[serde(rename(deserialize = "DKK"))]
    dkk: Ticker,
    #[serde(rename(deserialize = "EUR"))]
    eur: Ticker,
    #[serde(rename(deserialize = "GBP"))]
    gbp: Ticker,
    #[serde(rename(deserialize = "HKD"))]
    hkd: Ticker,
    #[serde(rename(deserialize = "HRK"))]
    hrk: Ticker,
    #[serde(rename(deserialize = "HUF"))]
    huf: Ticker,
    #[serde(rename(deserialize = "INR"))]
    inr: Ticker,
    #[serde(rename(deserialize = "ISK"))]
    isk: Ticker,
    #[serde(rename(deserialize = "JPY"))]
    jpy: Ticker,
    #[serde(rename(deserialize = "KRW"))]
    krw: Ticker,
    #[serde(rename(deserialize = "NZD"))]
    nzd: Ticker,
    #[serde(rename(deserialize = "PLN"))]
    pln: Ticker,
    #[serde(rename(deserialize = "RON"))]
    ron: Ticker,
    #[serde(rename(deserialize = "RUB"))]
    rub: Ticker,
    #[serde(rename(deserialize = "SEK"))]
    sek: Ticker,
    #[serde(rename(deserialize = "SGD"))]
    sgd: Ticker,
    #[serde(rename(deserialize = "THB"))]
    thb: Ticker,
    #[serde(rename(deserialize = "TRY"))]
    turkish_lira: Ticker,
    #[serde(rename(deserialize = "TWD"))]
    twd: Ticker,
    #[serde(rename(deserialize = "USD"))]
    usd: Ticker,
}

impl ApiConsumer {
    pub async fn fetch_data() -> Currencies {
        log::debug!("Request exchange rate data from {}", SOURCE_API);
        let error_message = format!("Unable to request data from {}!", SOURCE_API);
        let response = reqwest::get(SOURCE_API).await.expect(&error_message);
        log::debug!("Received response from {}", SOURCE_API);

        let error_message = format!("Unable to parse JSON from {}!", SOURCE_API);
        response.json().await.expect(&error_message)
    }
}

impl ExchangeRateApiConsumer for ApiConsumer {
    type Output = Pin<Box<dyn std::future::Future<Output = HashMap<Fiat, f64>>>>;

    fn fetch_api(&self) -> Self::Output {
        let fut = async {
            let currencies = Self::fetch_data().await;

            let mut map: HashMap<Fiat, f64> = HashMap::new();
            map.insert(Fiat::ARS, currencies.ars.last);
            map.insert(Fiat::AUD, currencies.aud.last);
            map.insert(Fiat::BRL, currencies.brl.last);
            map.insert(Fiat::CAD, currencies.cad.last);
            map.insert(Fiat::CHF, currencies.chf.last);
            map.insert(Fiat::CLP, currencies.clp.last);
            map.insert(Fiat::CNY, currencies.cny.last);
            map.insert(Fiat::CZK, currencies.czk.last);
            map.insert(Fiat::DKK, currencies.dkk.last);
            map.insert(Fiat::EUR, currencies.eur.last);
            map.insert(Fiat::GBP, currencies.gbp.last);
            map.insert(Fiat::HKD, currencies.hkd.last);
            map.insert(Fiat::HRK, currencies.hrk.last);
            map.insert(Fiat::HUF, currencies.huf.last);
            map.insert(Fiat::INR, currencies.inr.last);
            map.insert(Fiat::ISK, currencies.isk.last);
            map.insert(Fiat::JPY, currencies.jpy.last);
            map.insert(Fiat::KRW, currencies.krw.last);
            map.insert(Fiat::NZD, currencies.nzd.last);
            map.insert(Fiat::PLN, currencies.pln.last);
            map.insert(Fiat::RON, currencies.ron.last);
            map.insert(Fiat::RUB, currencies.rub.last);
            map.insert(Fiat::SEK, currencies.sek.last);
            map.insert(Fiat::SGD, currencies.sgd.last);
            map.insert(Fiat::THB, currencies.thb.last);
            map.insert(Fiat::TRY, currencies.turkish_lira.last);
            map.insert(Fiat::TWD, currencies.twd.last);
            map.insert(Fiat::USD, currencies.usd.last);

            map
        };

        Box::pin(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn api_call_must_not_fail() {
        ApiConsumer::fetch_data().await;
    }
}
