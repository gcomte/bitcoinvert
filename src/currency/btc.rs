use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::amount::Amount;
use crate::currency::{ConversionError, Currency};

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
    fn units_per_btc(&self) -> Result<Amount, ConversionError> {
        Ok(Amount::from_integer(match self {
            BitcoinUnit::BTC => 1,
            BitcoinUnit::MBTC => 1_000,
            BitcoinUnit::BITS => 1_000_000,
            BitcoinUnit::SAT => 100_000_000,
            BitcoinUnit::MSAT => 100_000_000_000,
        }))
    }

    fn decimal_places(&self) -> u8 {
        match self {
            BitcoinUnit::BTC => 11,
            BitcoinUnit::MBTC => 8,
            BitcoinUnit::BITS => 5,
            BitcoinUnit::SAT => 3,
            BitcoinUnit::MSAT => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::convert;
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
        fn millisatoshi_roundtrips_are_exact(
            millisatoshis in 0_u64..u64::MAX,
            unit in arb_btc_unit(),
        ) {
            let amount = Amount::from_integer(millisatoshis);
            let outputs: Vec<Box<dyn Currency>> = vec![Box::new(unit.clone())];
            let converted = convert(&amount, &BitcoinUnit::MSAT, &outputs, false).unwrap().remove(0);
            let outputs: Vec<Box<dyn Currency>> = vec![Box::new(BitcoinUnit::MSAT)];
            let restored = convert(&converted, &unit, &outputs, false).unwrap().remove(0);
            prop_assert_eq!(restored, amount);
        }

        #[test]
        fn exact_decimal_display_roundtrips(
            coefficient in i64::MIN..i64::MAX,
            exponent in -30..30,
        ) {
            let amount: Amount = format!("{coefficient}e{exponent}").parse().unwrap();
            let restored: Amount = amount.to_string().parse().unwrap();
            prop_assert_eq!(restored, amount);
        }

        #[test]
        fn conversion_preserves_order_at_millisatoshi_precision(
            a in 0_u64..u64::MAX,
            b in 0_u64..u64::MAX,
            unit in arb_btc_unit(),
        ) {
            let outputs: Vec<Box<dyn Currency>> = vec![Box::new(unit)];
            let a_converted = convert(&Amount::from_integer(a), &BitcoinUnit::MSAT, &outputs, false).unwrap().remove(0);
            let b_converted = convert(&Amount::from_integer(b), &BitcoinUnit::MSAT, &outputs, false).unwrap().remove(0);
            prop_assert_eq!(a.cmp(&b), a_converted.as_ratio().cmp(&b_converted.as_ratio()));
        }
    }

    #[test]
    fn all_bitcoin_units_preserve_a_millisatoshi() {
        let units: Vec<Box<dyn Currency>> = vec![
            Box::new(BitcoinUnit::BTC),
            Box::new(BitcoinUnit::MBTC),
            Box::new(BitcoinUnit::BITS),
            Box::new(BitcoinUnit::SAT),
            Box::new(BitcoinUnit::MSAT),
        ];
        let actual = convert(&Amount::from_integer(1), &BitcoinUnit::MSAT, &units, false).unwrap();
        assert_eq!(
            actual.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["0.00000000001", "0.00000001", "0.00001", "0.001", "1"]
        );
    }

    #[test]
    fn each_unit_rounds_half_a_millisatoshi_away_from_zero() {
        for amount in ["0.5", "-0.5"] {
            for unit in [
                BitcoinUnit::BTC,
                BitcoinUnit::MBTC,
                BitcoinUnit::BITS,
                BitcoinUnit::SAT,
                BitcoinUnit::MSAT,
            ] {
                let outputs: Vec<Box<dyn Currency>> = vec![Box::new(unit.clone())];
                let actual = convert(
                    &amount.parse().unwrap(),
                    &BitcoinUnit::MSAT,
                    &outputs,
                    false,
                )
                .unwrap()
                .remove(0);
                let outputs: Vec<Box<dyn Currency>> = vec![Box::new(BitcoinUnit::MSAT)];
                let restored = convert(&actual, &unit, &outputs, false).unwrap().remove(0);
                assert_eq!(
                    restored.to_string(),
                    if amount.starts_with('-') { "-1" } else { "1" }
                );
            }
        }
    }

    #[test]
    fn conversion_overflow_is_a_result_error() {
        let outputs: Vec<Box<dyn Currency>> = vec![Box::new(BitcoinUnit::MSAT)];
        assert!(matches!(
            convert(
                &"1e308".parse().unwrap(),
                &BitcoinUnit::BTC,
                &outputs,
                false
            ),
            Err(ConversionError::Arithmetic(_))
        ));
    }
}
