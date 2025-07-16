@echo off
setlocal enabledelayedexpansion

REM Root project directory
set ROOT=daxa

REM Create root project structure
mkdir %ROOT%
cd %ROOT%
echo [package]>Cargo.toml
echo fn main() {}>build.rs
echo all: >>Makefile

REM C Backend
mkdir c_backend
echo // dax_c.c placeholder > c_backend\dax_c.c
echo // dax_c.h placeholder > c_backend\dax_c.h

REM QML UI
mkdir dax_ui
echo // Explorer.qml placeholder > dax_ui\Explorer.qml
echo // main.qml placeholder > dax_ui\main.qml
echo // theme.qml placeholder > dax_ui\theme.qml
echo // Viewer.qml placeholder > dax_ui\Viewer.qml

REM Examples
mkdir examples
echo # Example Daxa file > examples\person.daxa
echo # Example Schema > examples\person_schema.toml

REM Rust src
mkdir src

REM CLI Submodule
mkdir src\cli
echo // convert.rs > src\cli\convert.rs
echo // extract.rs > src\cli\extract.rs
echo // info.rs > src\cli\info.rs
echo // mod.rs > src\cli\mod.rs
echo // pack.rs > src\cli\pack.rs
echo // validate.rs > src\cli\validate.rs

REM Daxa Format Submodule
mkdir src\dax_format
echo // mod.rs > src\dax_format\mod.rs
echo // parser.rs > src\dax_format\parser.rs
echo // value.rs > src\dax_format\value.rs
echo // writer.rs > src\dax_format\writer.rs

REM Schema Submodule
mkdir src\schema
echo // mod.rs > src\schema\mod.rs
echo // parser.rs > src\schema\parser.rs
echo // validator.rs > src\schema\validator.rs

REM Misc Rust modules
echo // compression.rs > src\compression.rs
echo // encryption.rs > src\encryption.rs
echo // gui_bridge.rs > src\gui_bridge.rs
echo // lib.rs > src\lib.rs
echo fn main() {} > src\main.rs
echo // mmap_io.rs > src\mmap_io.rs
echo // utils.rs > src\utils.rs

REM Tests
mkdir tests
echo // cli_tests.rs > tests\cli_tests.rs
echo // format_tests.rs > tests\format_tests.rs
echo // schema_tests.rs > tests\schema_tests.rs

echo.
echo ✅ Project structure for "Daxa" has been created successfully.
pause
