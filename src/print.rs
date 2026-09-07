use serde::Serialize;
use tabled::settings::Style;
use tabled::{Table, Tabled};
use thousands::Separable;

#[derive(Serialize)]
pub struct Money {
    pub amount: String,
    pub currency: String,
}

#[derive(Serialize)]
struct Conversion<'a> {
    input: &'a Money,
    outputs: &'a [Money],
}

#[derive(Tabled)]
struct TableRow {
    unit: String,
    amount: String,
}

pub fn multi_line(input: &Money, outputs: &[Money]) {
    let data = outputs.iter().map(|output| TableRow {
        unit: output.currency.clone(),
        amount: output.amount.separate_with_commas(),
    });

    let table = Table::new(data).with(Style::psql()).to_string();

    println!(
        "Input: {} {}\n\n{}",
        input.amount.separate_with_commas(),
        input.currency,
        table
    );
}

pub fn single_line(output: &Money, clean: bool) {
    if clean {
        println!("{}", output.amount);
    } else {
        println!(
            "{} {}",
            output.amount.separate_with_commas(),
            output.currency
        );
    }
}

pub fn json(input: &Money, outputs: &[Money]) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string(&Conversion { input, outputs })?;
    println!("{json}");
    Ok(())
}
