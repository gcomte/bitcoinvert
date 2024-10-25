use std::fmt::Display;

pub mod btc;
pub mod fiat;

#[typetag::serde()]
pub trait Currency: Display {
    fn btc_value(&self) -> f64;
    fn decimal_places(&self) -> u8;
}
