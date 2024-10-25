use std::fmt::Display;

pub mod btc;
pub mod fiat;

#[typetag::serde()]
pub trait Currency: Display {
    fn btc_value(&self) -> f64;
    fn decimal_places(&self) -> u8;
    fn round_value(&self, value: f64) -> f64 {
        let factor = 10_f64.powi(self.decimal_places().into());
        (value * factor).round() / factor
    }
}
