use clap::Parser;
use daxa_lib::cli::{Cli, Commands, handle_command}; // Use the lib's CLI module
use daxa_lib::Result;

fn main() -> Result<()> {
    // TODO: Initialize logging (e.g., env_logger or tracing)
    // env_logger::init();

    let cli = Cli::parse();
    match handle_command(cli) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{}", output);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}