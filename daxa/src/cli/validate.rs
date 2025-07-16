// validate.rs 
use std::path::Path;
use std::fs;
use crate::{Result, DaxaError};
use crate::dax_format;
use crate::schema::{Schema, validator};

pub fn run_validate(file_path: &Path, schema_path: Option<&Path>) -> Result<String> {
    let daxa_file = dax_format::load_daxa_file(file_path)?;
    
    let schema_to_use = if let Some(sp) = schema_path {
        let schema_content = fs::read_to_string(sp)?;
        // Assuming schema file is TOML for now
        Schema::from_toml_str(&schema_content)?
    } else {
        daxa_file.schema // Use embedded schema
    };

    if schema_to_use.types.is_empty() && schema_to_use.enums.is_empty() {
        return Ok("Schema is empty. No validation performed.".to_string());
    }

    let mut validation_errors = Vec::new();
    let mut datasets_validated = 0;

    // The current DaxaFile structure has `daxa_file.data: HashMap<String, DaxaValue>`.
    // The type information (e.g., `users: [User]`) is not directly stored alongside the DaxaValue.
    // This type info IS in the original `.daxa` text format example.
    // For validation, we need to know the *expected type* for each named dataset.
    // This information should ideally be part of the schema or parsed from the .daxa text.
    // For now, let's assume the schema contains top-level "dataset definitions" or we infer.
    //
    // Simplification: Let's assume the Daxa file's original text definition had something like:
    // `users: [User]` and `config_data: Config`.
    // We need a way to get "User" for "users" and "Config" for "config_data".
    // This mapping is missing in the current `DaxaFile` struct.
    //
    // Let's assume a convention: if a dataset is named "foos", its type is "Foo" (singular) or "[Foo]"
    // Or, the schema should have a top-level map of dataset names to their types.
    //
    // Let's try to find a struct/enum that matches the dataset name (or its singular form for arrays).
    // This is a heuristic and not robust.
    for (name, value) in &daxa_file.data {
        datasets_validated += 1;
        // Heuristic to guess the type name from the dataset name
        let mut type_name_guess = name.to_string();
        let mut is_array_guess = false;

        if matches!(value, crate::dax_format::DaxaValue::Array(_)) {
            is_array_guess = true;
            if name.ends_with('s') { // very naive plural to singular
                type_name_guess = name.trim_end_matches('s').to_string();
            }
            // Capitalize first letter for type name convention
            if let Some(first_char) = type_name_guess.chars().next() {
                type_name_guess = first_char.to_uppercase().to_string() + &type_name_guess[1..];
            }
        } else {
             if let Some(first_char) = type_name_guess.chars().next() {
                type_name_guess = first_char.to_uppercase().to_string() + &type_name_guess[1..];
            }
        }
        
        let full_type_name_str = if is_array_guess {
            format!("[{}]", type_name_guess)
        } else {
            type_name_guess
        };

        // Try to validate against the guessed type
        // This requires `validate_data_against_schema` to parse the `full_type_name_str` correctly.
        match validator::validate_data_against_schema(name, value, &full_type_name_str, &schema_to_use) {
            Ok(_) => {}
            Err(e) => validation_errors.push(format!("Dataset '{}' (guessed type '{}'): {}", name, full_type_name_str, e)),
        }
    }


    if validation_errors.is_empty() {
        if datasets_validated > 0 {
            Ok(format!("File '{}' is valid against the schema. ({} datasets validated)", file_path.display(), datasets_validated))
        } else {
             Ok(format!("File '{}' has no datasets to validate.", file_path.display()))
        }
    } else {
        Err(DaxaError::SchemaValidation(format!(
            "File '{}' has validation errors:\n{}",
            file_path.display(),
            validation_errors.join("\n")
        )))
    }
}