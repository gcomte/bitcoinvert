use serde::Deserialize;

const SOURCE_API: &str = "https://blockchain.info/ticker";

pub struct ApiConsumer;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(rename(deserialize = "15m"))]
    avg: f32,
    last: f32,
    buy: f32,
    sell: f32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn api_call_must_not_fail() {
        ApiConsumer::fetch_data().await;
    }
}
