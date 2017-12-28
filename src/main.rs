extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate ring;
extern crate base64;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate bigdecimal;

mod errors;
mod api;
mod num;

use bigdecimal::BigDecimal;

use api::API;
use errors::*;
use api::{Currency,Product};
use Currency::*;

fn main() {
    if let Err(err) = main2() {
        println!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let secrets = api::Secrets::from_file("secrets.yaml")?;
    let api = API::new(Some(secrets));
    for account in api.accounts()? {
        println!("Currency: {:?}", account.currency);
        println!("Balance: {:?} {:?}", account.balance, account.currency);
        if account.currency != USD {
            println!("Balance: ${} (market value)",
                     convert(&api, account.balance.val, account.currency, USD)?);
        }
        println!("");
    }
    Ok(())
}

fn convert(api: &API, amount: BigDecimal, from: Currency, to: Currency) -> Result<BigDecimal> {
    use std::str::FromStr;
    let book = api.book(Product{base: from, quote: to})?;
    let bid = book.bids.get(0).ok_or_else(|| "missing bid")?;
    let ask = book.asks.get(0).ok_or_else(|| "missing ask")?;
    let price = (bid.price.val.clone() + ask.price.val.clone()) / BigDecimal::from_str("2")?;
    Ok(amount * price)
}
