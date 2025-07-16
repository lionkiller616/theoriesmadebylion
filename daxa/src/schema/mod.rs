// mod.rs 
pub mod parser;
pub mod validator;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{Result, DaxaError};

#[deriveDebug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DaxaPrimitive {
    Bool,
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float32, Float64,
    String,
    Bytes,
    Datetime, // ISO 8601 string
    Uuid,
    // Any, // Special type for schemaless parts, handle with care
}

#[deriveDebug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DaxaType {
    Primitive(DaxaPrimitive),
    StructRef(String), // Name of a defined struct
    EnumRef(String),   // Name of a defined enum
    Array(Box<DaxaType>),
    Map(Box<DaxaType>, Box<DaxaType>), // Key type, Value type (key usually string or primitive)
    Optional(Box<DaxaType>),
    // TODO: Union(Vec<DaxaType>), // Tagged union
    // TODO: Custom(String), // User-defined opaque type
}

#[deriveDebug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub data_type: DaxaType,
    pub required: bool, // Alternative to DaxaType::Optional
    // TODO: pub attributes: HashMap<String, DaxaValue>, // e.g., @maxLength(100)
    // TODO: pub default_value: Option<DaxaValue>,
}

#[deriveDebug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<Field>,
    // TODO: pub version: u32,
}

#[deriveDebug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<String>,
    // TODO: pub version: u32,
}

#[deriveDebug, Clone, Default, Serialize, Deserialize)]
pub struct Schema {
    // Using HashMap for quick lookups by name
    pub types: HashMap<String, StructDefinition>, // Stores struct definitions
    pub enums: HashMap<String, EnumDefinition>,   // Stores enum definitions
    // TODO: version: String,
    // TODO: imports: Vec<String>, // For external schema files
}

impl Schema {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_struct(&mut self, def: StructDefinition) -> Result<()> {
        if self.types.contains_key(&def.name) || self.enums.contains_key(&def.name) {
            return Err(DaxaError::SchemaValidation(format!("Type or enum named '{}' already exists.", def.name)));
        }
        self.types.insert(def.name.clone(), def);
        Ok(())
    }

    pub fn add_enum(&mut self, def: EnumDefinition) -> Result<()> {
        if self.types.contains_key(&def.name) || self.enums.contains_key(&def.name) {
            return Err(DaxaError::SchemaValidation(format!("Type or enum named '{}' already exists.", def.name)));
        }
        self.enums.insert(def.name.clone(), def);
        Ok(())
    }

    pub fn get_struct(&self, name: &str) -> Option<&StructDefinition> {
        self.types.get(name)
    }

    pub fn get_enum(&self, name: &str) -> Option<&EnumDefinition> {
        self.enums.get(name)
    }

    // For .daxa text format writing
    pub fn to_toml_string(&self) -> Result<String> {
        // This will serialize the Schema struct itself as TOML.
        // A custom Daxa schema syntax would require a different serializer.
        toml::to_string_pretty(&self)
            .map_err(DaxaError::TomlSer)
    }

    // For .daxa text format reading (if schema is TOML)
    pub fn from_toml_str(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(DaxaError::TomlDe)
    }

    // TODO: Implement from_daxa_schema_str for Daxa's native schema syntax
    // TODO: Implement binary serialization/deserialization
    // TODO: Implement schema diffing and evolution/migration checks
}