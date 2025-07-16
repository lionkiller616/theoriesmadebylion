// schema_tests.rs 
use daxa_lib::schema::{Schema, StructDefinition, Field, DaxaType, DaxaPrimitive, EnumDefinition, validator};
use daxa_lib::dax_format::DaxaValue;
use std::collections::HashMap;

fn create_test_schema() -> Schema {
    let mut schema = Schema::new();
    schema.add_enum(EnumDefinition {
        name: "UserRole".to_string(),
        variants: vec!["Admin".to_string(), "Editor".to_string(), "Viewer".to_string()],
    }).unwrap();
    schema.add_struct(StructDefinition {
        name: "User".to_string(),
        fields: vec![
            Field { name: "id".to_string(), data_type: DaxaType::Primitive(DaxaPrimitive::UInt64), required: true },
            Field { name: "username".to_string(), data_type: DaxaType::Primitive(DaxaPrimitive::String), required: true },
            Field { name: "role".to_string(), data_type: DaxaType::EnumRef("UserRole".to_string()), required: true },
            Field { name: "email".to_string(), data_type: DaxaType::Optional(Box::new(DaxaType::Primitive(DaxaPrimitive::String))), required: false },
        ],
    }).unwrap();
    schema
}

#[test]
fn test_schema_creation() {
    let schema = create_test_schema();
    assert!(schema.get_struct("User").is_some());
    assert!(schema.get_enum("UserRole").is_some());
    assert_eq!(schema.get_struct("User").unwrap().fields.len(), 4);
}

#[test]
fn test_valid_user_data() {
    let schema = create_test_schema();
    let valid_user_data = DaxaValue::Struct("User".to_string(), [
        ("id".to_string(), DaxaValue::UInt64(1)),
        ("username".to_string(), DaxaValue::String("testuser".to_string())),
        ("role".to_string(), DaxaValue::String("Admin".to_string())), // Enum as string
        ("email".to_string(), DaxaValue::String("test@example.com".to_string())),
    ].iter().cloned().collect());

    // The validator needs a dataset name and its expected type string.
    // For a single struct, we can pass its name as the type string.
    let result = validator::validate_data_against_schema("test_user_dataset", &valid_user_data, "User", &schema);
    assert!(result.is_ok(), "Validation failed for valid data: {:?}", result.err());
}

#[test]
fn test_invalid_user_data_missing_field() {
    let schema = create_test_schema();
    let invalid_data = DaxaValue::Struct("User".to_string(),[
        ("id".to_string(), DaxaValue::UInt64(2)),
        // "username" is missing (required)
        ("role".to_string(), DaxaValue::String("Editor".to_string())),
    ].iter().cloned().collect());
    
    let result = validator::validate_data_against_schema("test_user_dataset", &invalid_data, "User", &schema);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Missing required field 'username'"));
    }
}

#[test]
fn test_invalid_user_data_wrong_type() {
    let schema = create_test_schema();
    let invalid_data = DaxaValue::Struct("User".to_string(), [
        ("id".to_string(), DaxaValue::String("not_a_u64".to_string())), // wrong type
        ("username".to_string(), DaxaValue::String("anotheruser".to_string())),
        ("role".to_string(), DaxaValue::String("Viewer".to_string())),
    ].iter().cloned().collect());

    let result = validator::validate_data_against_schema("test_user_dataset", &invalid_data, "User", &schema);
    assert!(result.is_err());
     if let Err(e) = result {
        assert!(e.to_string().contains("Type mismatch") || e.to_string().contains("does not match expected type"));
    }
}

#[test]
fn test_invalid_enum_variant() {
    let schema = create_test_schema();
    let invalid_data = DaxaValue::Struct("User".to_string(), [
        ("id".to_string(), DaxaValue::UInt64(3)),
        ("username".to_string(), DaxaValue::String("enumuser".to_string())),
        ("role".to_string(), DaxaValue::String("NonExistentRole".to_string())), // invalid enum
    ].iter().cloned().collect());

    let result = validator::validate_data_against_schema("test_user_dataset", &invalid_data, "User", &schema);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("is not a valid variant of enum 'UserRole'"));
    }
}

#[test]
fn test_optional_field_present_and_null() {
    let schema = create_test_schema();
    
    // Optional field present
    let data_with_email = DaxaValue::Struct("User".to_string(), [
        ("id".to_string(), DaxaValue::UInt64(4)),
        ("username".to_string(), DaxaValue::String("emailuser".to_string())),
        ("role".to_string(), DaxaValue::String("Viewer".to_string())),
        ("email".to_string(), DaxaValue::String("email@example.com".to_string())),
    ].iter().cloned().collect());
    let result1 = validator::validate_data_against_schema("test_user_dataset", &data_with_email, "User", &schema);
    assert!(result1.is_ok(), "Validation failed for optional field present: {:?}", result1.err());

    // Optional field explicitly null (if DaxaValue::Null is used for optional fields)
    // The DaxaType::Optional handles this.
    let data_with_null_email = DaxaValue::Struct("User".to_string(),[
        ("id".to_string(), DaxaValue::UInt64(5)),
        ("username".to_string(), DaxaValue::String("nullemailuser".to_string())),
        ("role".to_string(), DaxaValue::String("Viewer".to_string())),
        ("email".to_string(), DaxaValue::Null), // DaxaValue::Null for optional
    ].iter().cloned().collect());
    let result2 = validator::validate_data_against_schema("test_user_dataset", &data_with_null_email, "User", &schema);
    assert!(result2.is_ok(), "Validation failed for optional field as DaxaValue::Null: {:?}", result2.err());
    
    // Optional field omitted (validator logic should handle this if field.required = false)
    let data_without_email = DaxaValue::Struct("User".to_string(), [
        ("id".to_string(), DaxaValue::UInt64(6)),
        ("username".to_string(), DaxaValue::String("noemailuser".to_string())),
        ("role".to_string(), DaxaValue::String("Viewer".to_string())),
    ].iter().cloned().collect());
     let result3 = validator::validate_data_against_schema("test_user_dataset", &data_without_email, "User", &schema);
    assert!(result3.is_ok(), "Validation failed for optional field omitted: {:?}", result3.err());
}