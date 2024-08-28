use serde::{Deserialize, Serialize};

/// An enum of all the ways we can interpret values.
#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
  MessagePack(serde_json::Value),
  Json(serde_json::Value),
  String(String),
  Bytes(Vec<u8>),
}

impl Value {
  pub fn pretty(&self) -> String {
    match self {
      Value::MessagePack(v) => serde_json::to_string(v).unwrap(),
      Value::Json(v) => serde_json::to_string(v).unwrap(),
      Value::String(s) => format!("\"{}\"", s),
      Value::Bytes(b) => format!("{:#x}", hex_fmt::HexFmt(b)),
    }
  }

  pub fn pretty_long(&self) -> String {
    match self {
      Value::MessagePack(v) => serde_json::to_string_pretty(v).unwrap(),
      Value::Json(v) => serde_json::to_string_pretty(v).unwrap(),
      Value::String(s) => format!("\"{}\"", s),
      Value::Bytes(b) => format!("{:#x}", hex_fmt::HexFmt(b)),
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
