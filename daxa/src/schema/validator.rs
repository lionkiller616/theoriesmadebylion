// validator.rs 
use crate::schema::{Schema, DaxaType, DaxaPrimitive, StructDefinition, Field};
use crate::dax_format::DaxaValue;
use crate::{Result, DaxaError};
use std::collections::HashMap;

pub fn validate_data_against_schema(
    data_name: &str, // Name of the dataset, e.g., "users"
    data_value: &DaxaValue,
    expected_type_name: &str, // e.g., "[User]" or "User" or "Config"
    schema: &Schema,
) -> Result<()> {
    // 1. Parse the expected_type_name (e.g., "User", "array<User>", "map<string, User>")
    // This is simplified; a robust type string parser is needed.
    let base_type_name;
    let is_array;

    if expected_type_name.starts_with('[') && expected_type_name.ends_with(']') {
        is_array = true;
        base_type_name = expected_type_name.trim_start_matches('[').trim_end_matches(']').to_string();
        if !matches!(data_value, DaxaValue::Array(_)) {
            return Err(DaxaError::SchemaValidation(format!(
                "Data '{}' expected array of type '{}', found {:?}", data_name, base_type_name, data_value
            )));
        }
    } else {
        is_array = false;
        base_type_name = expected_type_name.to_string();
    }
    
    // 2. Resolve base_type_name to a DaxaType defined in the schema
    let resolved_type = schema.get_struct(&base_type_name)
        .map(|s| DaxaType::StructRef(s.name.clone())) // Represent as a reference for validation
        .or_else(|| schema.get_enum(&base_type_name).map(|e| DaxaType::EnumRef(e.name.clone())))
        .or_else(|| DaxaPrimitive::from_string(&base_type_name).map(DaxaType::Primitive));
        // TODO: Add map, optional, etc. parsing from type string

    let daxa_type_to_validate_against = match resolved_type {
        Some(dt) => dt,
        None => return Err(DaxaError::SchemaValidation(format!(
            "Type '{}' for data '{}' not found in schema.", base_type_name, data_name
        ))),
    };
    
    if is_array {
        if let DaxaValue::Array(items) = data_value {
            for (i, item) in items.iter().enumerate() {
                validate_value(item, &daxa_type_to_validate_against, schema)
                    .map_err(|e| DaxaError::SchemaValidation(format!("Error in array '{}' at index {}: {}", data_name, i, e)))?;
            }
        } else {
             // Already checked above, but good for completeness
            return Err(DaxaError::SchemaValidation(format!(
                "Data '{}' expected array, found non-array.", data_name
            )));
        }
    } else {
        validate_value(data_value, &daxa_type_to_validate_against, schema)?;
    }

    Ok(())
}


fn validate_value(value: &DaxaValue, expected_type: &DaxaType, schema: &Schema) -> Result<()> {
    match (value, expected_type) {
        (DaxaValue::Null, DaxaType::Optional(_)) => Ok(()),
        (DaxaValue::Null, _) => Err(DaxaError::SchemaValidation("Unexpected null value for non-optional type".into())),

        (DaxaValue::Bool(_), DaxaType::Primitive(DaxaPrimitive::Bool)) => Ok(()),
        (DaxaValue::Int8(_), DaxaType::Primitive(DaxaPrimitive::Int8)) => Ok(()),
        (DaxaValue::Int16(_), DaxaType::Primitive(DaxaPrimitive::Int16)) => Ok(()),
        (DaxaValue::Int32(_), DaxaType::Primitive(DaxaPrimitive::Int32)) => Ok(()),
        (DaxaValue::Int64(_), DaxaType::Primitive(DaxaPrimitive::Int64)) => Ok(()),
        (DaxaValue::UInt8(_), DaxaType::Primitive(DaxaPrimitive::UInt8)) => Ok(()),
        (DaxaValue::UInt16(_), DaxaType::Primitive(DaxaPrimitive::UInt16)) => Ok(()),
        (DaxaValue::UInt32(_), DaxaType::Primitive(DaxaPrimitive::UInt32)) => Ok(()),
        (DaxaValue::UInt64(_), DaxaType::Primitive(DaxaPrimitive::UInt64)) => Ok(()),
        (DaxaValue::Float32(_), DaxaType::Primitive(DaxaPrimitive::Float32)) => Ok(()),
        (DaxaValue::Float64(_), DaxaType::Primitive(DaxaPrimitive::Float64)) => Ok(()),
        (DaxaValue::String(_), DaxaType::Primitive(DaxaPrimitive::String)) => Ok(()), // TODO: Add attribute checks like maxLength
        (DaxaValue::Bytes(_), DaxaType::Primitive(DaxaPrimitive::Bytes)) => Ok(()),
        (DaxaValue::Timestamp(_), DaxaType::Primitive(DaxaPrimitive::Datetime)) => Ok(()),
        (DaxaValue::Uuid(_), DaxaType::Primitive(DaxaPrimitive::Uuid)) => Ok(()),

        (DaxaValue::Array(items), DaxaType::Array(inner_type)) => {
            for item in items {
                validate_value(item, inner_type, schema)?;
            }
            Ok(())
        }
        (DaxaValue::Map(map_data), DaxaType::Map(key_type, value_type)) => {
            // Assuming key_type is string for simplicity here as DaxaValue::Map keys are strings
            if !matches!(**key_type, DaxaType::Primitive(DaxaPrimitive::String)) {
                 return Err(DaxaError::SchemaValidation("Map keys must be strings in current DaxaValue::Map, schema expects other key type.".into()));
            }
            for (_key, val) in map_data { // Key already validated by DaxaValue::Map structure
                validate_value(val, value_type, schema)?;
            }
            Ok(())
        }
        (DaxaValue::Struct(name, fields_data), DaxaType::StructRef(expected_struct_name)) |
        (DaxaValue::Map(fields_data), DaxaType::StructRef(expected_struct_name)) // Allow map to validate as struct
         => {
            if let DaxaValue::Struct(name, _) = value {
                if name != expected_struct_name {
                     return Err(DaxaError::SchemaValidation(format!(
                        "Mismatched struct type. Expected '{}', found '{}'", expected_struct_name, name
                    )));
                }
            }

            let struct_def = schema.get_struct(expected_struct_name).ok_or_else(|| {
                DaxaError::SchemaValidation(format!("Struct definition '{}' not found in schema.", expected_struct_name))
            })?;

            validate_struct_fields(fields_data, &struct_def.fields, schema)
        }
        (DaxaValue::String(s), DaxaType::EnumRef(enum_name)) => {
            let enum_def = schema.get_enum(enum_name).ok_or_else(|| {
                DaxaError::SchemaValidation(format!("Enum definition '{}' not found in schema.", enum_name))
            })?;
            if !enum_def.variants.contains(s) {
                return Err(DaxaError::SchemaValidation(format!(
                    "Value '{}' is not a valid variant of enum '{}'. Valid variants: {:?}",
                    s, enum_name, enum_def.variants
                )));
            }
            Ok(())
        }
        (value, DaxaType::Optional(inner_type)) => {
            // If not null, validate against inner type
            validate_value(value, inner_type, schema)
        }

        // Mismatch
        (val, ty) => Err(DaxaError::SchemaValidation(format!(
            "Type mismatch. Value {:?} does not match expected type {:?}", val, ty
        ))),
    }
}

fn validate_struct_fields(
    data_fields: &HashMap<String, DaxaValue>,
    schema_fields: &[Field],
    schema: &Schema,
) -> Result<()> {
    for s_field in schema_fields {
        match data_fields.get(&s_field.name) {
            Some(value) => {
                validate_value(value, &s_field.data_type, schema)
                    .map_err(|e| DaxaError::SchemaValidation(format!("Error in field '{}': {}", s_field.name, e)))?;
            }
            None => {
                if s_field.required && !matches!(s_field.data_type, DaxaType::Optional(_)) {
                    return Err(DaxaError::SchemaValidation(format!(
                        "Missing required field '{}'", s_field.name
                    )));
                }
                // If not present and not required (or optional), it's fine.
                // Or if it has a default value (TODO)
            }
        }
    }

    // Optional: Check for extra fields in data not defined in schema
    for data_field_name in data_fields.keys() {
        if !schema_fields.iter().any(|sf| sf.name == *data_field_name) {
            // This could be an error or a warning depending on strictness
            // For now, let's allow extra fields as they might be in DaxaValue::Map
            // log::warn!("Extra field '{}' found in data not defined in struct schema.", data_field_name);
        }
    }
    Ok(())
}

impl DaxaPrimitive {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bool" | "boolean" => Some(DaxaPrimitive::Bool),
            "int8" => Some(DaxaPrimitive::Int8),
            "int16" => Some(DaxaPrimitive::Int16),
            "int32" | "int" => Some(DaxaPrimitive::Int32),
            "int64" | "long" => Some(DaxaPrimitive::Int64),
            "uint8" => Some(DaxaPrimitive::UInt8),
            "uint16" => Some(DaxaPrimitive::UInt16),
            "uint32" | "uint" => Some(DaxaPrimitive::UInt32),
            "uint64" | "ulong" => Some(DaxaPrimitive::UInt64),
            "float32" | "float" => Some(DaxaPrimitive::Float32),
            "float64" | "double" => Some(DaxaPrimitive::Float64),
            "string" | "str" => Some(DaxaPrimitive::String),
            "bytes" | "bytearray" | "blob" => Some(DaxaPrimitive::Bytes),
            "datetime" | "timestamp" => Some(DaxaPrimitive::Datetime),
            "uuid" => Some(DaxaPrimitive::Uuid),
            _ => None,
        }
    }
}