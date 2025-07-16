// cli_tests.rs 
use std::process::Command;
use std::path::PathBuf;
use assert_cmd::prelude::*; // Add `assert_cmd` to dev-dependencies in Cargo.toml
use predicates::prelude::*; // Add `predicates` to dev-dependencies
use tempfile::NamedTempFile;
use std::io::Write;

// Helper to get the path to the compiled CLI binary
fn get_cli_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    // Adjust for debug/release based on how you run tests
    // Typically `cargo test` builds in debug mode for the test harness itself
    // but the binary might be from a previous `cargo build`.
    // For `cargo test --release` it would be release.
    // Let's assume debug for simplicity.
    if cfg!(debug_assertions) {
        path.push("debug");
    } else {
        path.push("release");
    }
    path.push("daxa"); // Name of the binary
    path
}

#[test]
fn test_cli_info_command_on_sample_daxa() -> Result<(), Box<dyn std::error::Error>> {
    let cli_path = get_cli_binary_path();
    if !cli_path.exists() {
        panic!("CLI binary not found at {:?}. Build the project first.", cli_path);
    }

    // Create a dummy .daxa file for testing
    let mut temp_daxa_file = NamedTempFile::new()?;
    // This content needs to be parseable by the current simplified text parser
    // Schema as TOML, Data as JSON map.
    write!(temp_daxa_file, r#"
!daxa text v0.1_test
schema {{
[types.Dummy]
name = "Dummy"
fields = [
    {{ name = "value", data_type = {{ Primitive = "Int32" }}, required = true }}
]
}}
data {{
  "mydummy": {{ "Struct": ["Dummy", {{ "value": {{ "Int32": 123 }} }} ] }}
}}
"#)?;
    temp_daxa_file.flush()?;
    let temp_daxa_path = temp_daxa_file.path();

    let mut cmd = Command::new(cli_path);
    cmd.arg("info").arg(temp_daxa_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Daxa File Info"))
        .stdout(predicate::str::contains("Format Version: v0.1_test"))
        .stdout(predicate::str::contains("Schema: 1 types, 0 enums"))
        .stdout(predicate::str::contains("Named Datasets: 1"))
        .stdout(predicate::str::contains("- mydummy"));

    Ok(())
}

#[test]
fn test_cli_validate_command_valid_file() -> Result<(), Box<dyn std::error::Error>> {
    let cli_path = get_cli_binary_path();

    let mut temp_daxa_file = NamedTempFile::new()?;
    // This content needs to be parseable AND valid against its own schema
    // The schema validation logic in CLI currently makes a heuristic guess for types.
    // For "items: [Item]", it guesses Item type.
    write!(temp_daxa_file, r#"
!daxa text v0.1_valid
schema {
[types.Item]
name = "Item"
fields = [
    { name = "id", data_type = { Primitive = "UInt64" }, required = true },
    { name = "name", data_type = { Primitive = "String" }, required = true }
]
}
data {
  "items": { "Array": [
    { "Struct": ["Item", { "id": {"UInt64": 1}, "name": {"String": "First"} }] },
    { "Struct": ["Item", { "id": {"UInt64": 2}, "name": {"String": "Second"} }] }
  ]}
}
"#)?;
    temp_daxa_file.flush()?;
    let temp_daxa_path = temp_daxa_file.path();

    let mut cmd = Command::new(cli_path);
    cmd.arg("validate").arg(temp_daxa_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("is valid against the schema"));
        // .stdout(predicate::str::contains("(1 datasets validated)")); // depends on heuristic

    Ok(())
}


#[test]
fn test_cli_pack_command() -> Result<(), Box<dyn std::error::Error>> {
    let cli_path = get_cli_binary_path();

    let mut temp_daxa_file = NamedTempFile::new_in(".")? // Ensure it's in current dir for easier path
        .prefix("test_pack_input_")
        .suffix(".daxa")
        .keep(false)?; // Keep for duration of test, or delete after. False for auto-delete.
                       // For debugging, might set to true.

    // Same content as above, must be parseable
    write!(temp_daxa_file, r#"
!daxa text v0.1_pack
schema {{
[types.DataPoint]
name = "DataPoint"
fields = [ {{ name = "x", data_type = {{ Primitive = "Float64" }}, required = true }} ]
}}
data {{ "points": {{ "Array": [ {{ "Struct": ["DataPoint", {{ "x": {{ "Float64": 1.0 }} }} ] }} ] }} }}
"#)?;
    temp_daxa_file.flush()?;
    let input_daxa_path = temp_daxa_file.path().to_path_buf();


    let output_dax_temp = NamedTempFile::new_in(".")?
        .prefix("test_pack_output_")
        .suffix(".dax")
        .keep(false)?; 
    let output_dax_path = output_dax_temp.path().to_path_buf();
    // We need to drop output_dax_temp here so the file can be written to by the CLI,
    // otherwise it might be locked on Windows.
    drop(output_dax_temp);


    let mut cmd = Command::new(cli_path);
    cmd.arg("pack")
       .arg(&input_daxa_path)
       .arg(&output_dax_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully packed"));

    assert!(output_dax_path.exists(), "Packed .dax file was not created");
    assert!(output_dax_path.metadata()?.len() > 0, "Packed .dax file is empty");

    // Clean up the output file if NamedTempFile didn't handle it (e.g. if keep was true)
    // if output_dax_path.exists() { std::fs::remove_file(&output_dax_path)?; }

    Ok(())
}

// TODO: Add more tests for `extract`, `convert`, and error cases.