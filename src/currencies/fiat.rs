use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::currencies::Currency;
use crate::EXCHANGE_RATE_API_CONSUMER;

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
    HRK,
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
        unsafe { EXCHANGE_RATE_API_CONSUMER.btc_value(self) }
    }
}
