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
        if let Some(trace) = err.backtrace() {
            println!("{:?}", trace);
        }
        std::process::exit(1);
    }
}

struct Account {
    pub currency: Currency,
    pub balance: BigDecimal, // in currency units
    pub value_usd: BigDecimal, // in USD
}

fn main2() -> Result<()> {
    let secrets = api::Secrets::from_file("secrets.yaml")?;
    let api = API::new(Some(secrets));
    let mut accounts = Vec::new();
    for account in api.accounts()? {
        accounts.push(Account{
            currency:  account.currency,
            balance:   account.balance.val.clone(),
            value_usd: if account.currency != USD {
                convert(&api, account.balance.val, account.currency, USD)?
            } else {
                account.balance.val
            }
        });
    }
    for account in &accounts {
        println!("Currency: {:?}", account.currency);
        println!("Balance: {} {:?}", account.balance, account.currency);
        if account.currency != USD {
            println!("Balance: ${} (market value)", account.value_usd);
        }
        println!("");
    }
    let total_usd = accounts.iter().map(|a| a.value_usd.clone()).fold(num::zero(), |acc, b| acc + b);
    println!("total: ${} (market value)", total_usd);
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
