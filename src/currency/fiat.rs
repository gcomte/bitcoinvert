use crate::fiat_rates::blockchain_info_consumer;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::currency::Currency;
use crate::fiat_rates::exchange_rate_provider::ExchangeRateProvider;

// Static to have an easy way of caching the exchange rates.
static mut EXCHANGE_RATE_PROVIDER: ExchangeRateProvider<blockchain_info_consumer::ApiConsumer> =
    ExchangeRateProvider {
        data_source: blockchain_info_consumer::ApiConsumer,
        data: None,
    };

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum Fiat {
    ARS,
    AUD,
    BRL,
    CAD,
    CHF,
    CLP,
    CNY,
    CZK,
    DKK,
    EUR,
    GBP,
    HKD,
    HUF,
    INR,
    ISK,
    JPY,
    KRW,
    NZD,
    PLN,
    RON,
    RUB,
    SEK,
    SGD,
    THB,
    TRY,
    TWD,
    USD,
}

#[typetag::serde]
impl Currency for Fiat {
    fn btc_value(&self) -> f64 {
        unsafe { EXCHANGE_RATE_PROVIDER.btc_value(self) }
    }

    fn decimal_places(&self) -> u8 {
        match self {
            Fiat::AUD
            | Fiat::BRL
            | Fiat::CAD
            | Fiat::CHF
            | Fiat::CNY
            | Fiat::CZK
            | Fiat::DKK
            | Fiat::EUR
            | Fiat::GBP
            | Fiat::HKD
            | Fiat::INR
            | Fiat::NZD
            | Fiat::PLN
            | Fiat::RON
            | Fiat::RUB
            | Fiat::SEK
            | Fiat::SGD
            | Fiat::THB
            | Fiat::TRY
            | Fiat::TWD
            | Fiat::USD => 2,
            Fiat::ARS | Fiat::HUF | Fiat::JPY | Fiat::CLP | Fiat::ISK | Fiat::KRW => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_exchange_rate_caching() {
        let start = Instant::now();

        // First call should take a while.
        let btc_value = Fiat::USD.btc_value();
        let elapsed_first_call = start.elapsed().as_micros();
        assert!(btc_value > 0.0);
        assert!(elapsed_first_call > 1000);

        // Second call should be fast (even when looking up another fiat currency).
        let btc_value = Fiat::EUR.btc_value();
        assert!(btc_value > 0.0);
        assert!(start.elapsed().as_micros() - elapsed_first_call < 1000);
    }
}
