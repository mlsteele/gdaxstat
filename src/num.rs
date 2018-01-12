use std::fmt;
use std::str::FromStr;
use serde;
use serde::{Serialize,Serializer,Deserialize,Deserializer};
use serde::de::{Visitor};
pub use bigdecimal::BigDecimal;

pub fn zero() -> BigDecimal {
    BigDecimal::from_str("0").expect("zero value bigdecimal")
}

#[derive(Clone)]
pub struct BigDecimalField{
    pub val: BigDecimal
}

impl BigDecimalField {
    fn new(val: BigDecimal) -> Self {
        Self{val}
    }
}

impl fmt::Debug for BigDecimalField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Serialize for BigDecimalField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // TODO: Not sure this is correct (yes that seems bad)
        serializer.collect_str(&format!("{}", self.val))
    }
}

impl<'de> Deserialize<'de> for BigDecimalField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(BigDecimalFieldVisitor)
    }
}

struct BigDecimalFieldVisitor;

impl<'de> Visitor<'de> for BigDecimalFieldVisitor {
    type Value = BigDecimalField;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing a decimal number")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        use std::str::FromStr;
        use std::error::Error;
        BigDecimal::from_str(value)
            .map(|n| Self::Value::new(n))
            .map_err(|e| serde::de::Error::custom(e.description()))
    }
}
