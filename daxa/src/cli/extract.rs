// extract.rs 
use std::path::Path;
use crate::{Result, DaxaError};
use crate::dax_format::{self, DaxaValue};

// Very basic path extractor. Does not support complex queries like "users[?(@.age > 30)]"
// Supports: "dataset_name", "dataset_name[index]", "dataset_name.field", "dataset_name[index].field"
fn get_value_by_path<'a>(root_data: &'a std::collections::HashMap<String, DaxaValue>, path: &str) -> Option<&'a DaxaValue> {
    let mut parts = path.split('.');
    let first_part = parts.next()?;

    let (dataset_name, array_index) = if first_part.ends_with(']') {
        let open_bracket = first_part.rfind('[')?;
        let name = &first_part[..open_bracket];
        let index_str = &first_part[open_bracket+1 .. first_part.len()-1];
        let index = index_str.parse::<usize>().ok()?;
        (name, Some(index))
    } else {
        (first_part, None)
    };

    let mut current_value = root_data.get(dataset_name)?;

    if let Some(idx) = array_index {
        if let DaxaValue::Array(arr) = current_value {
            current_value = arr.get(idx)?;
        } else {
            return None; // Expected array
        }
    }

    for part in parts {
        let (field_name, array_index_inner) = if part.ends_with(']') {
            let open_bracket = part.rfind('[')?;
            let name = &part[..open_bracket];
            let index_str = &part[open_bracket+1 .. part.len()-1];
            let index = index_str.parse::<usize>().ok()?;
            (name, Some(index))
        } else {
            (part, None)
        };
        
        match current_value {
            DaxaValue::Map(map) | DaxaValue::Struct(_, map) => {
                current_value = map.get(field_name)?;
            }
            _ => return None, // Expected map or struct
        }

        if let Some(idx_inner) = array_index_inner {
             if let DaxaValue::Array(arr_inner) = current_value {
                current_value = arr_inner.get(idx_inner)?;
            } else {
                return None; // Expected array
            }
        }
    }
    Some(current_value)
}


pub fn run_extract(file_path: &Path, data_path: &str) -> Result<String> {
    let daxa_file = dax_format::load_daxa_file(file_path)?;

    match get_value_by_path(&daxa_file.data, data_path) {
        Some(value) => {
            // Output as JSON for now
            serde_json::to_string_pretty(&value.to_json_value())
                .map_err(|e| DaxaError::Serialization(format!("JSON serialization error: {}", e)))
        }
        None => Err(DaxaError::NotFound(format!(
            "Data path '{}' not found in file '{}'",
            data_path,
            file_path.display()
        ))),
    }
}
