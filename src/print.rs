use tabled::settings::Style;
use tabled::{Table, Tabled};
use thousands::Separable;

use crate::amount::Amount;
use crate::currency::Currency;

#[derive(Tabled)]
struct TableRow {
    unit: String,
    amount: String,
}

pub fn multi_line(output_values: &[Amount], currencies: &[Box<dyn Currency>]) {
    let mut data = Vec::new();

    for (output_value, currency) in output_values.iter().zip(currencies) {
        data.push(TableRow {
            unit: currency.to_string(),
            amount: output_value.separate_with_commas().to_string(),
        });
    }

    let table = Table::new(data).with(Style::psql()).to_string();

    println!("{}", table);
}

pub fn single_line(output_value: &Amount, currency: &dyn Currency, clean: bool) {
    if clean {
        println!("{}", output_value);
    } else {
        println!("{} {}", output_value.separate_with_commas(), currency);
    }
}
