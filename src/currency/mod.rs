use std::fmt::Display;

use crate::amount::{Amount, AmountError};

pub mod btc;
pub mod fiat;

#[typetag::serde()]
pub trait Currency: Display {
    /// Units of this currency per BTC, kept as the exact decimal source rate.
    fn units_per_btc(&self) -> Result<Amount, ConversionError>;
    fn decimal_places(&self) -> u8;
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Conversion failed: {0}")]
    Arithmetic(#[from] AmountError),
    #[error("Exchange rate for {0} must be a positive finite decimal number")]
    InvalidRate(String),
    #[error("No exchange rate is available for {0}")]
    MissingRate(String),
    #[error("Unable to load exchange rates: {0}")]
    RateSource(String),
}

/// Convert all outputs before printing, so a failed rate or overflow cannot
/// leave an apparently successful partial result on stdout. Input and rate
/// bounds limit all temporary ratios; division is only by positive rates.
pub fn convert(
    amount: &Amount,
    input_currency: &dyn Currency,
    output_currencies: &[Box<dyn Currency>],
    integer: bool,
) -> Result<Vec<Amount>, ConversionError> {
    let input_rate = input_currency.units_per_btc()?;
    if !input_rate.is_positive() {
        return Err(ConversionError::InvalidRate(input_currency.to_string()));
    }
    let value_in_btc = amount.as_ratio() / input_rate.as_ratio();
    output_currencies
        .iter()
        .map(|currency| {
            let output_rate = currency.units_per_btc()?;
            if !output_rate.is_positive() {
                return Err(ConversionError::InvalidRate(currency.to_string()));
            }
            let precision = if integer {
                0
            } else {
                currency.decimal_places()
            };
            Ok(Amount::from_ratio_rounded(
                value_in_btc.clone() * output_rate.as_ratio(),
                precision,
            )?)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::btc::BitcoinUnit;
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Serialize, Deserialize)]
    struct FixedCurrency {
        rate: Amount,
        places: u8,
    }

    impl Display for FixedCurrency {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("TEST")
        }
    }

    #[typetag::serde(name = "test_fixed_rate")]
    impl Currency for FixedCurrency {
        fn units_per_btc(&self) -> Result<Amount, ConversionError> {
            Ok(self.rate.clone())
        }

        fn decimal_places(&self) -> u8 {
            self.places
        }
    }

    fn fixed(rate: &str, places: u8) -> Box<dyn Currency> {
        Box::new(FixedCurrency {
            rate: rate.parse().unwrap(),
            places,
        })
    }

    #[test]
    fn fiat_midpoints_use_exact_source_rates() {
        for (input, expected) in [("1", "50000.01"), ("-1", "-50000.01")] {
            let outputs = vec![fixed("50000.005", 2)];
            let actual =
                convert(&input.parse().unwrap(), &BitcoinUnit::BTC, &outputs, false).unwrap();
            assert_eq!(actual[0].to_string(), expected);
        }
    }

    #[test]
    fn fiat_cross_rates_round_only_the_final_result() {
        // 1 / 3 * 1.005 = exactly 0.335. Rounding 1 / 3 prematurely
        // would incorrectly put this midpoint below the boundary.
        let input = fixed("3", 2);
        let outputs = vec![fixed("1.005", 2)];
        let actual = convert(&Amount::from_integer(1), &*input, &outputs, false).unwrap();
        assert_eq!(actual[0].to_string(), "0.34");
    }

    #[test]
    fn identical_fiat_rate_cancels_exactly() {
        let input = fixed("62345.12345678901234567890123456789", 2);
        let outputs = vec![fixed("62345.12345678901234567890123456789", 2)];
        for amount in ["1.005", "-1.005", "9007199254740993.005"] {
            let parsed: Amount = amount.parse().unwrap();
            let actual = convert(&parsed, &*input, &outputs, false).unwrap();
            let expected = Amount::from_ratio_rounded(parsed.as_ratio(), 2).unwrap();
            assert_eq!(actual[0], expected);
        }
    }

    #[test]
    fn zero_rates_cannot_reach_division() {
        for rate in ["0", "-1"] {
            let bad = fixed(rate, 2);
            let outputs: Vec<Box<dyn Currency>> = vec![Box::new(BitcoinUnit::BTC)];
            assert!(matches!(
                convert(&Amount::from_integer(1), &*bad, &outputs, false),
                Err(ConversionError::InvalidRate(_))
            ));
            assert!(matches!(
                convert(&Amount::from_integer(1), &BitcoinUnit::BTC, &[bad], false),
                Err(ConversionError::InvalidRate(_))
            ));
        }
    }

    #[test]
    fn integer_rounding_also_uses_midpoint_away_from_zero() {
        let outputs = vec![fixed("2.5", 2)];
        for (input, expected) in [("1", "3"), ("-1", "-3")] {
            let actual =
                convert(&input.parse().unwrap(), &BitcoinUnit::BTC, &outputs, true).unwrap();
            assert_eq!(actual[0].to_string(), expected);
        }
    }
}
