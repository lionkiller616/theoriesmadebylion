// pack.rs 
use std::path::Path;
use crate::{Result, DaxaError};
use crate::dax_format;

pub fn run_pack(input_path: &Path, output_path: &Path) -> Result<String> {
    if input_path.extension().and_then(|s| s.to_str()) != Some("daxa") {
        return Err(DaxaError::InvalidFormat("Input for pack must be a .daxa text file.".into()));
    }
    if output_path.extension().and_then(|s| s.to_str()) != Some("dax") {
         // Could also auto-add extension if missing
        return Err(DaxaError::InvalidFormat("Output for pack should be a .dax binary file.".into()));
    }

    let daxa_file_content = dax_format::load_daxa_file(input_path)?;

    // TODO: Add options for compression, encryption algorithm selection if specified by user

    dax_format::save_daxa_file(output_path, &daxa_file_content, true)?; // true for binary

    Ok(format!(
        "Successfully packed '{}' into binary Daxa file '{}'",
        input_path.display(),
        output_path.display()
    ))
}