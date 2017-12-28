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
use api::API;

use errors::*;

fn main() {
    if let Err(err) = main2() {
        println!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let secrets = api::Secrets::from_file("secrets.yaml")?;
    let api = API::new(Some(secrets));
    println!("{:#?}", api.accounts()?);
    Ok(())
}
