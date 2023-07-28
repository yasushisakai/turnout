use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;
use hex::{ToHex, FromHex};

use crate::error::TurnoutError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Address([u8; 32]);

impl Address {
   pub fn as_bytes(&self) -> &[u8] {
       &self.0
   }
}

impl TryFrom<[u8;32]> for Address {
    type Error = TurnoutError;

    fn try_from(value: [u8;32]) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl TryFrom<String> for Address {
    type Error = TurnoutError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = <[u8;32]>::from_hex(value).or(Err(TurnoutError::AddressConversion))?;
        Ok(Self(bytes))
    }
}

impl fmt::Display for Address{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex: String = self.0.encode_hex();
        f.write_str(&hex)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex: String = self.0.encode_hex();
        serializer.serialize_str(&hex)
    }
}

struct AddressVisior;

impl<'de> Visitor<'de> for AddressVisior {
    type Value = Address;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex representation of a sha256 digest")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = <[u8; 32]>::from_hex(v).or(Err(E::custom("cannot convert hex representation to byte array")))?;
        Ok(Address(bytes))
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AddressVisior)
    }
}

impl std::hash::Hash for Address {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bytes: [u8; std::mem::size_of::<u64>()] =
            self.0[..std::mem::size_of::<u64>()].try_into().unwrap();
        let n = u64::from_be_bytes(bytes);
        state.write_u64(n);
    }
}

impl nohash::IsEnabled for Address {}
impl nohash::IsEnabled for &Address {}
