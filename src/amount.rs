//! Exact decimal amounts with bounded parsing and arithmetic.
//!
//! Input is limited before constructing big integers. Finite decimal amounts
//! have at most 309 integer digits and 308 fractional places; powers of ten
//! are bounded independently so scientific notation cannot allocate unbounded
//! integers. Conversion uses exact ratios and rounds only its final result.

use std::fmt;
use std::str::FromStr;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub const MAX_AMOUNT_BYTES: usize = 1024;
const MAX_INTEGER_DIGITS: usize = 309;
const MAX_SCALE: u32 = 308;
const MAX_EXPONENT: i32 = 308;
const MAX_RATIO_BITS: u64 = 8192;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount {
    coefficient: BigInt,
    scale: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum AmountError {
    #[error("expected a finite decimal number")]
    Invalid,
    #[error("amount is outside the supported range (309 integer digits, 308 fractional places, exponent -308 to 308, at most 1024 input bytes)")]
    OutOfRange,
}

impl Amount {
    pub fn from_integer(value: u64) -> Self {
        Self {
            coefficient: BigInt::from(value),
            scale: 0,
        }
    }

    fn new(mut coefficient: BigInt, mut scale: u32) -> Result<Self, AmountError> {
        // Normalize once so output never has trailing decimal zeros or -0.
        if coefficient.is_zero() {
            return Ok(Self::from_integer(0));
        }
        while scale > 0 && (&coefficient % 10_u8).is_zero() {
            coefficient /= 10_u8;
            scale -= 1;
        }
        let digits = coefficient.abs().to_str_radix(10).len();
        if scale > MAX_SCALE || digits.saturating_sub(scale as usize) > MAX_INTEGER_DIGITS {
            return Err(AmountError::OutOfRange);
        }
        Ok(Self { coefficient, scale })
    }

    pub fn is_positive(&self) -> bool {
        self.coefficient.is_positive()
    }

    pub(crate) fn as_ratio(&self) -> BigRational {
        BigRational::new(
            self.coefficient.clone(),
            BigInt::from(10_u8).pow(self.scale),
        )
    }

    /// Round a bounded ratio once, using nearest with midpoint away from zero.
    pub(crate) fn from_ratio_rounded(
        value: BigRational,
        decimal_places: u8,
    ) -> Result<Self, AmountError> {
        if value.numer().bits() > MAX_RATIO_BITS || value.denom().bits() > MAX_RATIO_BITS {
            return Err(AmountError::OutOfRange);
        }
        let scale = u32::from(decimal_places);
        let scaled = value.numer() * BigInt::from(10_u8).pow(scale);
        let mut coefficient = &scaled / value.denom();
        let remainder = (&scaled % value.denom()).abs();
        if remainder * 2_u8 >= *value.denom() {
            coefficient += if scaled.is_negative() { -1_i8 } else { 1_i8 };
        }
        Self::new(coefficient, scale)
    }

    /// Parse a decimal and apply an SI exponent without rounding either step.
    pub fn parse_with_exponent(input: &str, si_exponent: i8) -> Result<Self, AmountError> {
        if input.len() > MAX_AMOUNT_BYTES {
            return Err(AmountError::OutOfRange);
        }
        let (mantissa, exponent) = match input.split_once(['e', 'E']) {
            Some((mantissa, exponent)) => {
                let exponent = exponent.parse::<i32>().map_err(|_| AmountError::Invalid)?;
                if !(-MAX_EXPONENT..=MAX_EXPONENT).contains(&exponent) {
                    return Err(AmountError::OutOfRange);
                }
                (mantissa, exponent)
            }
            None => (input, 0),
        };
        let exponent = exponent + i32::from(si_exponent);
        if !(-MAX_EXPONENT..=MAX_EXPONENT).contains(&exponent) {
            return Err(AmountError::OutOfRange);
        }
        let (negative, unsigned) = match mantissa.as_bytes().first() {
            Some(b'-') => (true, &mantissa[1..]),
            Some(b'+') => (false, &mantissa[1..]),
            _ => (false, mantissa),
        };
        let mut coefficient_digits = String::with_capacity(unsigned.len());
        let mut fractional_places = None;
        for byte in unsigned.bytes() {
            match byte {
                b'0'..=b'9' => {
                    coefficient_digits.push(char::from(byte));
                    if let Some(places) = &mut fractional_places {
                        *places += 1_i32;
                    }
                }
                b'.' if fractional_places.is_none() => fractional_places = Some(0),
                _ => return Err(AmountError::Invalid),
            }
        }
        if coefficient_digits.is_empty() {
            return Err(AmountError::Invalid);
        }
        let mut coefficient =
            BigInt::parse_bytes(coefficient_digits.as_bytes(), 10).ok_or(AmountError::Invalid)?;
        if negative {
            coefficient = -coefficient;
        }
        let scale = fractional_places.unwrap_or(0) - exponent;
        if scale < 0 {
            // `exponent` is already bounded, as is the mantissa length.
            coefficient *= BigInt::from(10_u8).pow(scale.unsigned_abs());
            Self::new(coefficient, 0)
        } else {
            Self::new(coefficient, scale as u32)
        }
    }
}

impl FromStr for Amount {
    type Err = AmountError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse_with_exponent(input, 0)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = self.coefficient.abs().to_str_radix(10);
        if self.coefficient.is_negative() {
            f.write_str("-")?;
        }
        let scale = self.scale as usize;
        if scale == 0 {
            f.write_str(&digits)
        } else if digits.len() <= scale {
            write!(f, "0.{}{}", "0".repeat(scale - digits.len()), digits)
        } else {
            let split = digits.len() - scale;
            write!(f, "{}.{}", &digits[..split], &digits[split..])
        }
    }
}

impl Serialize for Amount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AmountVisitor;
        impl de::Visitor<'_> for AmountVisitor {
            type Value = Amount;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an exact decimal amount")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                value.parse().map_err(E::custom)
            }
        }
        // Callers must preserve numeric lexemes. YAML uses no_schema mode;
        // the API uses serde_json::Number with arbitrary_precision enabled.
        // Deliberately do not implement visit_f64: it would lose precision.
        deserializer.deserialize_str(AmountVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_exact_decimals_and_scientific_notation() {
        for (input, expected) in [
            ("9007199254740993", "9007199254740993"),
            (
                "0.12345678901234567890123456789",
                "0.12345678901234567890123456789",
            ),
            ("1.005e-2", "0.01005"),
            ("1.25E+3", "1250"),
            ("+.50", "0.5"),
            ("-0.000", "0"),
        ] {
            assert_eq!(input.parse::<Amount>().unwrap().to_string(), expected);
        }
        assert_eq!(
            Amount::parse_with_exponent("0.001005", 3)
                .unwrap()
                .to_string(),
            "1.005"
        );
        assert_eq!(
            Amount::parse_with_exponent("1e-305", -3).unwrap().scale,
            308
        );
    }

    #[test]
    fn rejects_nonfinite_and_unbounded_input() {
        for input in [
            "",
            "+",
            ".",
            "NaN",
            "inf",
            "-Infinity",
            "1e",
            "1e1e1",
            "1..0",
            "1e999999999",
            "1e309",
            "1e-309",
        ] {
            assert!(input.parse::<Amount>().is_err(), "{input}");
        }
        assert!("1".repeat(MAX_AMOUNT_BYTES + 1).parse::<Amount>().is_err());
        assert!("9"
            .repeat(MAX_INTEGER_DIGITS + 1)
            .parse::<Amount>()
            .is_err());
        assert!(Amount::parse_with_exponent("1e308", 3).is_err());
        assert!("1e308".parse::<Amount>().is_ok());
        assert!("1e-308".parse::<Amount>().is_ok());
    }

    #[test]
    fn midpoint_rounding_is_away_from_zero() {
        for (input, places, expected) in [
            ("1.005", 2, "1.01"),
            ("-1.005", 2, "-1.01"),
            ("0.0005", 3, "0.001"),
            ("-0.0004", 3, "0"),
            ("2.5", 0, "3"),
            ("-2.5", 0, "-3"),
        ] {
            let amount: Amount = input.parse().unwrap();
            assert_eq!(
                Amount::from_ratio_rounded(amount.as_ratio(), places)
                    .unwrap()
                    .to_string(),
                expected
            );
        }
    }
}
