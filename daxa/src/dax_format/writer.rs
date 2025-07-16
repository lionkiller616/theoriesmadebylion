// writer.rs 
use crate::dax_format::{DaxaFile, DaxaValue, DAXA_TEXT_MAGIC, DAXA_BINARY_MAGIC};
use crate::schema::Schema; // Assuming Schema serialization lives in schema module
use crate::{DaxaError, Result};
use std::io::Write;
use byteorder::{WriteBytesExt, LittleEndian}; // Or BigEndian

pub fn write_daxa_text_content<W: Write>(writer: &mut W, daxa_file: &DaxaFile) -> Result<()> {
    // Write magic and version
    writer.write_all(DAXA_TEXT_MAGIC)?;
    writer.write_all(daxa_file.format_version.as_bytes())?;
    writer.write_all(b"\n\n")?;

    // Write schema block
    writer.write_all(b"schema {\n")?;
    // This assumes Schema has a to_toml_string() or similar.
    // A custom Daxa schema syntax would need its own serializer.
    let schema_str = daxa_file.schema.to_toml_string()
        .map_err(|e| DaxaError::Serialization(format!("Schema serialization error: {}", e)))?;
    writer.write_all(schema_str.as_bytes())?;
    writer.write_all(b"}\n\n")?;

    // Write data block
    writer.write_all(b"data {\n")?;
    // This is a placeholder. Daxa's custom data syntax needs its own serializer.
    // For now, let's serialize data as a JSON map of named datasets.
    // This would NOT match the `users: [User] = [...]` example directly.
    // That would require a custom data serializer that understands the schema context.
    for (name, value) in &daxa_file.data {
        // A proper Daxa text writer would format this according to the Daxa syntax.
        // e.g., users: [User] = [ { id: 1, ... }, ... ];
        // This is a huge simplification for now, just writing as key = json_value;
        let json_val_str = serde_json::to_string_pretty(&value.to_json_value())?;
        writer.write_all(format!("  {}: ", name).as_bytes())?;
        // Determine type from schema for proper formatting (e.g. `users: [User] =`)
        // For now, we just dump the value.
        writer.write_all(json_val_str.as_bytes())?; 
        writer.write_all(b";\n")?; // Using semicolon as an example terminator
    }
    writer.write_all(b"}\n")?;

    Ok(())
}

pub fn write_dax_binary_content<W: Write>(writer: &mut W, daxa_file: &DaxaFile) -> Result<()> {
    writer.write_all(DAXA_BINARY_MAGIC)?;

    // TODO: Implement full binary writing:
    // 1. Write global header (version, compression flags, encryption flags, placeholders for offsets)
    // 2. Serialize and write schema (potentially compressed/encrypted)
    // 3. Serialize and write data blocks (potentially compressed/encrypted)
    //    - Use internal indexing/jump tables
    // 4. Update header with actual schema/data/index offsets and sizes
    // This is a major piece of work.

    // Placeholder for writing schema (binary format)
    // daxa_file.schema.write_binary(writer)?;

    // Placeholder for writing data (binary format)
    // for (name, value) in &daxa_file.data {
    //    write_named_value_binary(writer, name, value, &daxa_file.schema)?;
    // }

    Ok(())
}

// Helper (stub)
fn _write_named_value_binary<W: Write>(_writer: &mut W, _name: &str, _value: &DaxaValue, _schema: &Schema) -> Result<()> {
    // This would involve:
    // - Looking up type information for _name in _schema
    // - Writing the value according to its DaxaType, using binary encoding rules
    //   (e.g., varints for integers, length-prefixed strings, etc.)
    // - Handling compression and encryption per block if configured
    Ok(())
}

pub fn save_daxa_file(path: &std::path::Path, daxa_file: &DaxaFile, is_binary: bool) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    if is_binary {
        write_dax_binary_content(&mut file, daxa_file)
    } else {
        write_daxa_text_content(&mut file, daxa_file)
    }
}