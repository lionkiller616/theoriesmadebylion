// convert.rs 
use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::{Result, DaxaError};
use crate::dax_format::{self, DaxaValue};

pub fn run_convert(
    file_path: &Path,
    to_format: &str,
    output_path: Option<&Path>,
    dataset_name: Option<&str>,
) -> Result<String> {
    let daxa_file = dax_format::load_daxa_file(file_path)?;

    let value_to_convert = if let Some(ds_name) = dataset_name {
        daxa_file.data.get(ds_name)
            .ok_or_else(|| DaxaError::NotFound(format!("Dataset '{}' not found.", ds_name)))?
    } else {
        // If no specific dataset, and multiple datasets, this might be ambiguous for CSV.
        // For JSON/YAML, we can convert the whole data map.
        if daxa_file.data.len() == 1 {
            daxa_file.data.values().next().unwrap() // Get the single dataset
        } else if to_format == "json" || to_format == "yaml" {
            // Convert all data if format supports multiple top-level entries well.
            // We need to wrap the HashMap into a DaxaValue::Map to use to_json_value.
            let whole_data_map: std::collections::HashMap<String, DaxaValue> = daxa_file.data.clone();
             // This is a bit of a hack. The DaxaValue itself should represent the whole file's data.
             // For now, we'll create a temporary DaxaValue::Map.
            let temp_value = DaxaValue::Map(whole_data_map);
            // This is problematic because we return a reference from an owned value.
            // We need to actually process `temp_value` here.
            // Let's handle this directly in the match block.
             return convert_value(&DaxaValue::Map(daxa_file.data), to_format, output_path, Some(file_path.to_string_lossy().to_string()));
        } else {
            return Err(DaxaError::Unsupported(
                "Multiple datasets found. Please specify a --dataset for CSV conversion, or convert to JSON/YAML.".to_string()
            ));
        }
    };

    convert_value(value_to_convert, to_format, output_path, dataset_name.map(String::from))
}


fn convert_value(
    value: &DaxaValue,
    to_format: &str,
    output_path: Option<&Path>,
    name_hint: Option<String> // For CSV header/filename hint
) -> Result<String> {
    let output_str = match to_format {
        "json" => serde_json::to_string_pretty(&value.to_json_value())?,
        "yaml" => serde_yaml::to_string(&value.to_json_value())
            .map_err(|e| DaxaError::Serialization(format!("YAML serialization error: {}", e)))?,
        "csv" => convert_to_csv(value, name_hint.as_deref())?,
        _ => return Err(DaxaError::Unsupported(format!("Conversion to '{}' is not supported.", to_format))),
    };

    if let Some(out_p) = output_path {
        let mut file = File::create(out_p)?;
        file.write_all(output_str.as_bytes())?;
        Ok(format!("Successfully converted to {} and saved to {}", to_format, out_p.display()))
    } else {
        Ok(output_str) // Return as string for stdout
    }
}


fn convert_to_csv(value: &DaxaValue, _name_hint: Option<&str>) -> Result<String> {
    let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);

    match value {
        DaxaValue::Array(records) if !records.is_empty() => {
            // Assuming array of structs/maps
            if let Some(first_record) = records.first() {
                match first_record {
                    DaxaValue::Struct(_, fields) | DaxaValue::Map(fields) => {
                        let mut headers: Vec<String> = fields.keys().cloned().collect();
                        headers.sort(); // Consistent column order
                        wtr.write_record(&headers)?;

                        for record_value in records {
                            if let DaxaValue::Struct(_, current_fields) | DaxaValue::Map(current_fields) = record_value {
                                let mut row = Vec::new();
                                for header in &headers {
                                    // Convert DaxaValue to simple string for CSV
                                    let cell_value = current_fields.get(header)
                                        .map_or(String::new(), |dv| daxa_value_to_csv_string(dv));
                                    row.push(cell_value);
                                }
                                wtr.write_record(&row)?;
                            } else {
                                return Err(DaxaError::Unsupported("CSV conversion expects an array of uniform objects/maps.".into()));
                            }
                        }
                    }
                    _ => return Err(DaxaError::Unsupported("CSV conversion expects an array of objects/maps.".into())),
                }
            } else { // Empty array
                // Write empty CSV or just headers? For now, empty.
            }
        }
        _ => return Err(DaxaError::Unsupported(format!("CSV conversion expects an array of objects/maps, found {:?}", value))),
    }

    String::from_utf8(wtr.into_inner()
        .map_err(|e| DaxaError::Serialization(format!("CSV writer error: {}", e)))?)
        .map_err(|e| DaxaError::Serialization(format!("CSV UTF-8 error: {}", e)))
}

fn daxa_value_to_csv_string(dv: &DaxaValue) -> String {
    match dv {
        DaxaValue::Null => String::new(),
        DaxaValue::Bool(b) => b.to_string(),
        DaxaValue::Int8(i) => i.to_string(),
        DaxaValue::Int16(i) => i.to_string(),
        DaxaValue::Int32(i) => i.to_string(),
        DaxaValue::Int64(i) => i.to_string(),
        DaxaValue::UInt8(i) => i.to_string(),
        DaxaValue::UInt16(i) => i.to_string(),
        DaxaValue::UInt32(i) => i.to_string(),
        DaxaValue::UInt64(i) => i.to_string(),
        DaxaValue::Float32(f) => f.to_string(),
        DaxaValue::Float64(f) => f.to_string(),
        DaxaValue::String(s) => s.clone(),
        DaxaValue::Timestamp(ts) => ts.to_rfc3339(),
        DaxaValue::Uuid(u) => u.to_string(),
        DaxaValue::Bytes(b) => base64::encode(b), // Bytes as base64
        DaxaValue::Array(_) | DaxaValue::Map(_) | DaxaValue::Struct(_, _) => {
            // Nested complex types represented as JSON string in CSV cell
            serde_json::to_string(&dv.to_json_value()).unwrap_or_else(|_| "ERR_SERIALIZE".to_string())
        }
    }
}