use crate::fiat_rates::blockchain_info_consumer;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};
use strum_macros::{Display, EnumString};

use crate::amount::Amount;
use crate::currency::{ConversionError, Currency};
use crate::fiat_rates::exchange_rate_provider::ExchangeRateProvider;

// Static to have an easy way of caching the exchange rates.
static EXCHANGE_RATE_PROVIDER: LazyLock<
    Mutex<ExchangeRateProvider<blockchain_info_consumer::ApiConsumer>>,
> = LazyLock::new(|| {
    Mutex::new(ExchangeRateProvider {
        data_source: blockchain_info_consumer::ApiConsumer,
        data: None,
    })
});

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
    GHS,
    HKD,
    HUF,
    INR,
    ISK,
    JPY,
    KRW,
    NGN,
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
    fn units_per_btc(&self) -> Result<Amount, ConversionError> {
        EXCHANGE_RATE_PROVIDER
            .lock()
            .map_err(|_| ConversionError::RateSource("rate cache lock was poisoned".to_string()))?
            .units_per_btc(self)
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
            | Fiat::GHS
            | Fiat::HKD
            | Fiat::INR
            | Fiat::NGN
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

    #[test]
    fn live_fiat_rates_are_positive() {
        // Cache reuse is tested deterministically by ExchangeRateProvider's
        // fetch counter; this smoke test exercises the real Fiat path.
        for currency in [Fiat::USD, Fiat::EUR] {
            assert!(currency.units_per_btc().unwrap().is_positive());
        }
    }
}
