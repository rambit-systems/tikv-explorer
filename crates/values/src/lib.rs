use std::fmt;

use serde::{Deserialize, Serialize};

/// An enum of all the ways we can interpret values.
#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
  MessagePack(serde_json::Value),
  Json(serde_json::Value),
  String(String),
  Bytes(Vec<u8>),
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::MessagePack(v) => {
        write!(f, "{}", serde_json::to_string(v).unwrap())
      }
      Value::Json(v) => write!(f, "{}", serde_json::to_string(v).unwrap()),
      Value::String(s) => write!(f, "\"{}\"", s),
      Value::Bytes(b) => write!(f, "{:#x}", hex_fmt::HexFmt(b)),
    }
  }
}

impl From<Vec<u8>> for Value {
  fn from(bytes: Vec<u8>) -> Self {
    // check all the possible representations
    if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
      return Value::Json(value);
    }

    // try to convert to a string
    if let Ok(string) = String::from_utf8(bytes.clone()) {
      return Value::String(string);
    }

    if let Ok(value) = rmp_serde::from_slice::<serde_json::Value>(&bytes) {
      return Value::MessagePack(value);
    }

    Value::Bytes(bytes)
  }
}
