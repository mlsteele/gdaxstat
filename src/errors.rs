use std;
use reqwest;
use serde_yaml;
use base64;
use bigdecimal;

// use std;
// use std::borrow::Borrow;

// pub type Result<T> = std::result::Result<T, String>;

// pub fn e<T,S>(msg: S) -> Result<T>
//     where S: Borrow<str>
// {
//     return Err(msg.borrow().to_string());
// }

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        SerdeYaml(serde_yaml::Error);
        B64Decode(base64::DecodeError);
        ParseBigDecimal(bigdecimal::ParseBigDecimalError);
    }
}
