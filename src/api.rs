use std::fs::File;
use reqwest;
use reqwest::{Url,Method};
use reqwest::header::{Headers,ContentType,UserAgent};
use chrono::{ Utc};
use ring;
use base64;
use serde_yaml;
use num::BigDecimalField;

use errors::*;

pub struct API {
    pub root: Url,
    secrets: Option<Secrets>,
}

impl API {
    pub fn new(secrets: Option<Secrets>) -> Self {
        Self{
            root: Url::parse("https://api.gdax.com").expect("invalid root uri"),
            secrets: secrets,
        }
    }

    pub fn accounts(&self) -> Result<Vec<Account>> {
        let req_path = "/accounts";
        let url = self.root.join(req_path).chain_err(|| "url parse")?;
        let body = "{}".to_owned();
        let client = reqwest::Client::new();
        let req = client.request(Method::Get, url)
            .headers(self.private_headers("GET", req_path, &body)?)
            .body(body).build()?;
        let mut resp = client.execute(req)?;
        if !resp.status().is_success() {
            bail!("request failed: {} ({})", resp.status(), resp.text()?);
        }
        let obj: Vec<Account> = resp.json()?;
        Ok(obj)
    }

    fn headers(&self) -> Headers {
        let mut h = Headers::new();
        h.set(UserAgent::new("gdaxstat/0.1.0"));
        h.set(ContentType::json());
        h
    }

    fn private_headers(&self, method: &str, req_path: &str, body: &str) -> Result<Headers> {
        let secrets = self.get_secrets()?;
        let mut h = self.headers();
        let t = Self::timestamp();
        let sig = self.sign(&t, method, req_path, body)?;
        h.set_raw("CB-ACCESS-TIMESTAMP", t);
        h.set_raw("CB-ACCESS-KEY", secrets.api_key.clone());
        h.set_raw("CB-ACCESS-SIGN", sig);
        h.set_raw("CB-ACCESS-PASSPHRASE", secrets.passphrase.clone());
        Ok(h)
    }

    fn sign(&self, t: &str, method: &str, req_path: &str, body: &str) -> Result<String> {
        let payload = format!("{}{}{}{}", t, method, req_path, body);
        let key = ring::hmac::SigningKey::new(&ring::digest::SHA256, self.get_secrets()?.api_secret.as_ref());
        let sig = ring::hmac::sign(&key, payload.as_ref());
        Ok(base64::encode(&sig))
    }

    fn timestamp() -> String {
        format!("{}", Utc::now().format("%s%.6f"))
    }

    fn get_secrets(&self) -> Result<&Secrets> {
        self.secrets.as_ref().ok_or_else(|| "missing secrets".into())
    }
}

pub type AccountID = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Currency {
    USD,
    BTC,
    BCH,
    ETH,
    LTC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountID,
    pub currency: Currency,
    pub balance: BigDecimalField,
}

pub struct Secrets {
    api_key: String,
    api_secret: Vec<u8>,
    passphrase: String,
}

impl Secrets {
    pub fn from_file(path: &str) -> Result<Secrets> {
        let f = File::open(path)?;
        let data: SecretsData = serde_yaml::from_reader(f)?;
        Ok(Secrets{
            api_key: data.api_key,
            api_secret: base64::decode(&data.api_secret)?,
            passphrase: data.passphrase,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct SecretsData {
    api_key: String,
    api_secret: String, // base64-encoded
    passphrase: String,
}

