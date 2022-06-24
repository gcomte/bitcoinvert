mod blockchain_info_consumer;
pub mod cli_input;
pub mod currencies;
pub mod defaults;
pub mod exchange_rate_provider;

use crate::cli_input::CliInput;
use crate::currencies::Currency;
use crate::exchange_rate_provider::ExchangeRateProvider;

// Okay this is so bad.
// The initial idea was to make crate::currencies::Currency having a generic implementing
// crate::exchange_rate_provider::ExchangeRateApiConsumer, to fetch the exchange rates.
// The problem is that typetag::serde() does not (yet) support generics.
// Adding the crate::exchange_rate_provider::ExchangeRateApiConsumer directly to crate::currencies::Fiat
// does not work either, because Fiat is an enum.
static mut EXCHANGE_RATE_API_CONSUMER: ExchangeRateProvider<blockchain_info_consumer::ApiConsumer> =
    ExchangeRateProvider {
        data_source: blockchain_info_consumer::ApiConsumer,
        data: None,
    };

#[tokio::main]
async fn main() {
    env_logger::init();

    unsafe {
        EXCHANGE_RATE_API_CONSUMER.fetch().await;
    }

    let cli_input = CliInput::parse();

    println!("input currency: {}", cli_input.amount);
    println!("input currency: {:?}", cli_input.input_currency.to_string());

    // first: let's get the amount of sats.

    // each currency has an amount of sats it's worth.

    let mut x = ExchangeRateProvider {
        data_source: blockchain_info_consumer::ApiConsumer,
        data: None,
    };

    x.fetch().await;

    // println!("{:?}: {}", Fiat::CHF, x.btc_value(&Fiat::CHF));

    // println!(
    //     "output currencies: {:?}",
    //     cli_input
    //         .output_currencies
    //         .iter()
    //         .map(|c| format!("{}: {}", &c, &c.btc_value()))
    //         .collect::<Vec<String>>()
    // );

    let value_in_btc = cli_input.amount * cli_input.input_currency.btc_value();

    for currency in cli_input.output_currencies {
        println!("{}: {}", currency, value_in_btc / currency.btc_value());
    }
}
