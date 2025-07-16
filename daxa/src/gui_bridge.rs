// gui_bridge.rs 
#[cxx_qt::bridge(cxx_file_stem = "daxa_gui_bridge")]
pub mod ffi {
    // Re-export types from daxa_lib that you want to use in QML or GUI logic
    use crate::{DaxaFile, DaxaValue, Schema, DaxaError, Result as DaxaResult};
    use crate::dax_format;
    use std::path::PathBuf;

    #[derive(Debug, Clone, Default, qmetaobject::SimpleListItem)] // For QML lists
    pub struct QmlDaxaTypeItem {
        pub name: String,
        pub kind: String, // "Struct", "Enum"
        pub field_count: i32, // For structs
    }

    #[derive(Debug, Clone, Default, qmetaobject::SimpleListItem)]
    pub struct QmlDaxaDataItem {
        pub name: String,
        pub item_type: String, // e.g. "User", "[User]", "Config"
        pub item_count: i32,  // e.g. for arrays
        // pub preview: String, // A short string preview of the data
    }


    #[cxx_qt::qobject(qml_uri = "com.daxa.gui", qml_version = "1.0")]
    pub struct DaxaViewModel {
        #[qproperty]
        file_path: QString,
        #[qproperty]
        status_message: QString,
        #[qproperty]
        is_file_loaded: bool,

        // Using cxx_qt_lib::QVariantList for complex data to QML
        // Simpler: QListOfStruct<QmlDaxaTypeItem> if using that.
        // For simplicity, let's try with QListOfStruct for now if QML can handle it.
        // However, QVariantList is more flexible.
        // Let's use Rust Vec and convert to QVariantList manually if needed or see if cxx-qt handles it.
        // Update: cxx-qt prefers its own list types for direct QML list models.
        // For complex structs, you might need to make them QObjects themselves or use QVariantMap.
        
        // Let's try with simple properties for now, and complex lists later.
        #[qproperty]
        schema_summary: QString, // Placeholder for schema display
        #[qproperty]
        data_summary: QString,   // Placeholder for data display

        // Store the loaded DaxaFile internally
        #[qmetaobject:: ગામ(Найстарший)] // Marking as internal field, not directly a qproperty
        daxa_file_internal: Option<DaxaFile>,
    }

    impl Default for DaxaViewModel {
        fn default() -> Self {
            Self {
                file_path: QString::from(""),
                status_message: QString::from("No file loaded."),
                is_file_loaded: false,
                schema_summary: QString::from("Schema: N/A"),
                data_summary: QString::from("Data: N/A"),
                daxa_file_internal: None,
            }
        }
    }

    impl qmetaobject::QSingleton for DaxaViewModel {
        fn const_default() -> Option<Self> {
            Some(Self::default())
        }
    }


    impl DaxaViewModel {
        #[qinvokable]
        pub fn load_daxa_file(self: Pin<&mut Self>, path: QString) {
            let rust_path_str = path.to_string();
            let p = PathBuf::from(rust_path_str.clone()); // Keep path string for messages

            // Ensure we get a mutable reference to self via cxx-qt's mechanism
            let mut pinned_self = self.pin_mut();

            pinned_self.set_file_path(path); // Update property

            match dax_format::load_daxa_file(&p) {
                Ok(loaded_file) => {
                    pinned_self.set_status_message(QString::from("File loaded successfully."));
                    pinned_self.set_is_file_loaded(true);
                    
                    let schema_info = format!(
                        "Schema: {} types, {} enums.",
                        loaded_file.schema.types.len(),
                        loaded_file.schema.enums.len()
                    );
                    pinned_self.set_schema_summary(QString::from(&schema_info));

                    let data_info = format!(
                        "Data: {} named datasets.",
                        loaded_file.data.len()
                    );
                    pinned_self.set_data_summary(QString::from(&data_info));
                    
                    // Store the loaded file
                    pinned_self.as_mut().daxa_file_internal = Some(loaded_file);
                }
                Err(e) => {
                    let err_msg = format!("Error loading file '{}': {}", rust_path_str, e);
                    pinned_self.set_status_message(QString::from(&err_msg));
                    pinned_self.set_is_file_loaded(false);
                    pinned_self.set_schema_summary(QString::from("Schema: N/A"));
                    pinned_self.set_data_summary(QString::from("Data: N/A"));
                    pinned_self.as_mut().daxa_file_internal = None;
                }
            }
        }

        // TODO: Add more invokable methods to get schema details, data items, etc.
        // For example, to populate a QML ListView for schema types:
        /*
        #[qinvokable]
        pub fn get_schema_types(self: Pin<&mut Self>) -> cxx_qt_lib::QVariantList {
            let mut list = cxx_qt_lib::QVariantList::default();
            if let Some(file) = &self.daxa_file_internal {
                for (name, def) in &file.schema.types {
                    let mut map = cxx_qt_lib::QVariantMap::default();
                    map.insert(QString::from("name"), QVariant::from(QString::from(name)));
                    map.insert(QString::from("kind"), QVariant::from(QString::from("Struct")));
                    map.insert(QString::from("field_count"), QVariant::from(def.fields.len() as i32));
                    list.append(QVariant::from(map));
                }
                // ... also for enums
            }
            list
        }
        */

        #[qinvokable]
        pub fn get_file_path_for_dialog(&self) -> QString {
            // QFileDialog needs a native path string
            // For simplicity, just return the current path, assuming it's usable.
            // On some platforms, QML PathUrl or local file strings might need conversion.
            self.file_path.clone()
        }
    }
}