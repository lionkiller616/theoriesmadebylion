// value.rs 
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[deriveDebug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DaxaValue {
    Null,
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<DaxaValue>),
    Map(HashMap<String, DaxaValue>), // For schemaless maps or struct-like data
    Struct(String, HashMap<String, DaxaValue>), // Struct name and its fields
    Timestamp(DateTime<Utc>),
    Uuid(Uuid),
    // TODO: EmbeddedFile(String, Vec<u8>), // name, content
    // TODO: Custom(String, Vec<u8>), // Custom type name and its binary data
}

// Basic conversions for CLI output, etc.
impl DaxaValue {
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            DaxaValue::Null => serde_json::Value::Null,
            DaxaValue::Bool(b) => serde_json::Value::Bool(*b),
            DaxaValue::Int8(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::Int16(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::Int32(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::Int64(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::UInt8(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::UInt16(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::UInt32(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::UInt64(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DaxaValue::Float32(f) => serde_json::Number::from_f64(*f as f64).map_or(serde_json::Value::Null, serde_json::Value::Number),
            DaxaValue::Float64(f) => serde_json::Number::from_f64(*f).map_or(serde_json::Value::Null, serde_json::Value::Number),
            DaxaValue::String(s) => serde_json::Value::String(s.clone()),
            DaxaValue::Bytes(b) => serde_json::Value::String(base64::encode(b)), // Represent bytes as base64 string in JSON
            DaxaValue::Array(arr) => serde_json::Value::Array(arr.iter().map(|v| v.to_json_value()).collect()),
            DaxaValue::Map(map) | DaxaValue::Struct(_, map) => {
                let mut json_map = serde_json::Map::new();
                for (k, v) in map {
                    json_map.insert(k.clone(), v.to_json_value());
                }
                serde_json::Value::Object(json_map)
            }
            DaxaValue::Timestamp(ts) => serde_json::Value::String(ts.to_rfc3339()),
            DaxaValue::Uuid(u) => serde_json::Value::String(u.to_string()),
        }
    }
}