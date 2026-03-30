use tabled::settings::Style;
use tabled::{Table, Tabled};
use thousands::Separable;

use crate::currency::Currency;

#[derive(Tabled)]
struct TableRow {
    unit: String,
    amount: String,
}

pub fn multi_line(value_in_btc: f64, currencies: &[Box<dyn Currency>], integer: bool) {
    let mut data = Vec::new();

    for currency in currencies {
        let mut output_value = value_in_btc / currency.btc_value();
        if integer {
            output_value = output_value.round();
        } else {
            output_value = currency.round_value(output_value);
        }

        data.push(TableRow {
            unit: currency.to_string(),
            amount: output_value.separate_with_commas().to_string(),
        });
    }

    let table = Table::new(data).with(Style::psql()).to_string();

    println!("{}", table);
}

pub fn single_line(output_value: f64, currency: &dyn Currency, clean: bool) {
    if clean {
        println!("{}", output_value);
    } else {
        println!("{} {}", output_value.separate_with_commas(), currency);
    }
}
