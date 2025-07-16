// mod.rs 
pub mod parser;
pub mod writer;
pub mod value;

pub use value::DaxaValue;
use crate::schema::Schema;
use std::collections::HashMap;

// Constants for Daxa format
pub const DAXA_TEXT_MAGIC: &[u8; 12] = b"!daxa text "; // followed by version e.g. v1
pub const DAXA_BINARY_MAGIC: &[u8; 8] = b"DAXBIN\x01\x00"; // Example binary magic + version
pub const DAX_EXTENSION: &str = ".dax";
pub const DAXA_EXTENSION: &str = ".daxa";
pub const DAXM_EXTENSION: &str = ".daxm";
pub const DAXLOG_EXTENSION: &str = ".daxlog";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DaxaFileFormat {
    Text,
    Binary,
    MemoryMappedBinary, // .daxm implies binary, special loading
    AppendLog,          // .daxlog implies binary, special handling
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DaxaFile {
    pub format_version: String,
    pub schema: Schema,
    pub data: HashMap<String, DaxaValue>, // Named datasets
    // TODO: metadata, compression_info, encryption_info, jump_tables, etc.
}

impl DaxaFile {
    pub fn new(format_version: String, schema: Schema) -> Self {
        DaxaFile {
            format_version,
            schema,
            data: HashMap::new(),
        }
    }
}