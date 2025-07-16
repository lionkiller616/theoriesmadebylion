fn main() {
    // Build C backend
    cc::Build::new()
        .file("c_backend/dax_c.c")
        .compiler("gcc") // or clang
        .flag("-O3")
        .flag("-march=native") // Be cautious with this for cross-platform distribution
        .compile("dax_c_lib");

    println!("cargo:rustc-link-lib=static=dax_c_lib");
    println!("cargo:rerun-if-changed=c_backend/dax_c.c");
    println!("cargo:rerun-if-changed=c_backend/dax_c.h");

    // Generate Rust bindings for C code
    let bindings = bindgen::Builder::default()
        .header("c_backend/dax_c.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for dax_c.h");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("c_bindings.rs"))
        .expect("Couldn't write C bindings!");

    // Qt/QML integration with cxx-qt
    // This will look for cxx_qt::bridge modules in your Rust code
    // and generate the necessary C++ files.
    cxx_qt_build::CxxQtBuilder::new()
        .file("src/gui_bridge.rs") // Tell cxx-qt about our bridge module
        .qml_module("com.daxa.gui", "1.0", "dax_ui") // Register QML module
        .setup_linker() // Link Qt libraries
        .build();

    println!("cargo:rerun-if-changed=src/gui_bridge.rs");
    println!("cargo:rerun-if-changed=dax_ui/main.qml");
    println!("cargo:rerun-if-changed=dax_ui/Viewer.qml");
    println!("cargo:rerun-if-changed=dax_ui/Explorer.qml");
    println!("cargo:rerun-if-changed=dax_ui/theme.qml");
}