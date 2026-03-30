use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::currency::Currency;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum BitcoinUnit {
    BTC,  // bitcoin
    MBTC, // milli-bitcoin
    BITS, // μBTC, micro-bitcoin
    SAT,  // satoshi
    MSAT, // milli-satoshi
}

#[typetag::serde]
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

    fn decimal_places(&self) -> u8 {
        match self {
            BitcoinUnit::BTC => 8,
            BitcoinUnit::MBTC => 5,
            BitcoinUnit::BITS => 2,
            BitcoinUnit::SAT => 3,
            BitcoinUnit::MSAT => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_btc_unit() -> impl Strategy<Value = BitcoinUnit> {
        prop_oneof![
            Just(BitcoinUnit::BTC),
            Just(BitcoinUnit::MBTC),
            Just(BitcoinUnit::BITS),
            Just(BitcoinUnit::SAT),
            Just(BitcoinUnit::MSAT),
        ]
    }

    proptest! {
        #[test]
        fn roundtrip_btc_conversion(
            amount in 1.0e-8_f64..1.0e12,
            from in arb_btc_unit(),
            to in arb_btc_unit(),
        ) {
            // Convert from -> BTC -> to -> BTC -> from
            let in_btc = amount * from.btc_value();
            let in_target = in_btc / to.btc_value();
            let back_in_btc = in_target * to.btc_value();
            let back_in_from = back_in_btc / from.btc_value();

            let relative_error = ((back_in_from - amount) / amount).abs();
            prop_assert!(
                relative_error < 1.0e-10,
                "Roundtrip error too large: {amount} -> {back_in_from} (error: {relative_error})"
            );
        }

        #[test]
        fn btc_value_is_positive(unit in arb_btc_unit()) {
            prop_assert!(unit.btc_value() > 0.0);
        }

        #[test]
        fn conversion_preserves_order(
            a in 1.0_f64..1.0e12,
            b in 1.0_f64..1.0e12,
            unit in arb_btc_unit(),
        ) {
            // If a > b in one unit, a > b in any other unit
            let a_btc = a * unit.btc_value();
            let b_btc = b * unit.btc_value();
            prop_assert_eq!(a > b, a_btc > b_btc);
        }

        #[test]
        fn round_value_error_within_half_unit(
            amount in 0.0_f64..1.0e8,
            unit in arb_btc_unit(),
        ) {
            let rounded = unit.round_value(amount);
            let half_unit = 0.5 / 10_f64.powi(unit.decimal_places().into());
            // Allow a small extra tolerance for f64 representation at large magnitudes
            let tolerance = half_unit + amount.abs() * f64::EPSILON;
            let error = (rounded - amount).abs();
            prop_assert!(
                error <= tolerance,
                "round_value({amount}) = {rounded}, error {error} exceeds half unit {half_unit}"
            );
        }
    }
}
