use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::currencies::Currency;

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum BitcoinUnit {
    BTC,  // bitcoin
    MBTC, // milli-bitcoin
    BITS, // μBTC, micro-bitcoin
    SAT,  // satoshi
    MSAT, // milli-satoshi
}

#[typetag::serde]
#[async_trait]
impl Currency for BitcoinUnit {
    fn btc_value(&self) -> f64 {
        match &self {
            BitcoinUnit::BTC => 1.0,
            BitcoinUnit::MBTC => 0.001,
            BitcoinUnit::BITS => 0.000_001,
            BitcoinUnit::SAT => 0.000_000_01,
            BitcoinUnit::MSAT => 0.000_000_000_01,
        }
    }
}
