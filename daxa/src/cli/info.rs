// info.rs 
use std::path::Path;
use crate::{Result, DaxaError};
use crate::dax_format;

pub fn run_info(file_path: &Path) -> Result<String> {
    let daxa_file = dax_format::load_daxa_file(file_path)?;

    // TODO: Extract more detailed info from binary files (blocks, compression, etc.)
    let mut output = String::new();
    output.push_str(&format!("Daxa File Info: {}\n", file_path.display()));
    output.push_str(&format!("  Format Version: {}\n", daxa_file.format_version));
    output.push_str(&format!("  Schema: {} types, {} enums\n", daxa_file.schema.types.len(), daxa_file.schema.enums.len()));
    if !daxa_file.schema.types.is_empty() {
        output.push_str("    Types:\n");
        for name in daxa_file.schema.types.keys() {
            output.push_str(&format!("      - {}\n", name));
        }
    }
    if !daxa_file.schema.enums.is_empty() {
        output.push_str("    Enums:\n");
        for name in daxa_file.schema.enums.keys() {
            output.push_str(&format!("      - {}\n", name));
        }
    }
    output.push_str(&format!("  Named Datasets: {}\n", daxa_file.data.len()));
    for (name, value) in &daxa_file.data {
        let item_count = match value {
            crate::dax_format::DaxaValue::Array(arr) => format!(" ({} items)", arr.len()),
            _ => String::new(),
        };
        output.push_str(&format!("    - {}{}\n", name, item_count));
    }

    // TODO: For binary files, add:
    // - Total size
    // - Number of data blocks
    // - Compression status (per block or global)
    // - Encryption status
    // - Indexing information

    Ok(output)
}