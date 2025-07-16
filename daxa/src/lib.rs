// lib.rs 
pub mod cli;
pub mod compression;
pub mod dax_format;
pub mod encryption;
pub mod mmap_io;
pub mod schema;
pub mod utils;
pub mod gui_bridge; // For cxx-qt

// Re-export key types and functions for easier use by the CLI or other consumers
pub use dax_format::{DaxaFile, DaxaValue};
pub use schema::{Schema, DaxaType, Field};

// Include generated C bindings
// mod c_bindings {
//     #![allow(non_upper_case_globals)]
//     #![allow(non_camel_case_types)]
//     #![allow(non_snake_case)]
//     #![allow(dead_code)]
//     include!(concat!(env!("OUT_DIR"), "/c_bindings.rs"));
// }
// pub use c_bindings::*; // Expose C functions if needed directly

#[derive(Debug, thiserror::Error)]
pub enum DaxaError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parsing Error: {0}")]
    Parsing(String),
    #[error("Serialization Error: {0}")]
    Serialization(String),
    #[error("Schema Validation Error: {0}")]
    SchemaValidation(String),
    #[error("Compression Error: {0}")]
    Compression(String),
    #[error("Encryption Error: {0}")]
    Encryption(String),
    #[error("Unsupported Feature: {0}")]
    Unsupported(String),
    #[error("Invalid File Format: {0}")]
    InvalidFormat(String),
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("Data Not Found: {0}")]
    NotFound(String),
    #[error("Toml Deserialization Error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Toml Serialization Error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("JSON Serialization Error: {0}")]
    JsonSer(#[from] serde_json::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, DaxaError>;