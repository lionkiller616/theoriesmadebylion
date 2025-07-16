// mod.rs 
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::{Result, DaxaError}; // Use crate's Result and DaxaError
use crate::dax_format::{self, DaxaFile}; // Corrected path
use crate::schema::{validator, Schema};    // Corrected path
use crate::dax_format::value::DaxaValue; // Corrected path

mod info;
mod validate;
mod extract;
mod convert;
mod pack;

#[derive(Parser, Debug)]
#[command(author, version, about = "Daxa: The Ultimate Data Storage Language CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display summary information about a .dax or .daxa file
    Info {
        /// Path to the Daxa file
        #[arg(required = true)]
        file_path: PathBuf,
    },
    /// Validate a Daxa file against its embedded or an external schema
    Validate {
        /// Path to the Daxa file
        #[arg(required = true)]
        file_path: PathBuf,
        /// Optional path to an external schema file (e.g., .toml or .daxa-schema)
        #[arg(long)]
        schema_path: Option<PathBuf>,
    },
    /// Extract a specific key, record, or data path from a Daxa file
    Extract {
        /// Path to the Daxa file
        #[arg(required = true)]
        file_path: PathBuf,
        /// Data path to extract (e.g., "users[0].name" or "config.settings.theme")
        #[arg(required = true)]
        data_path: String,
    },
    /// Convert a Daxa file to another format (JSON, YAML, CSV)
    Convert {
        /// Path to the Daxa file
        #[arg(required = true)]
        file_path: PathBuf,
        /// Target format
        #[arg(long, value_parser = ["json", "yaml", "csv"])] // Add more as supported
        to: String,
        /// Optional: Output file path (stdout if not provided)
        #[arg(long, short)]
        output: Option<PathBuf>,
        /// Optional: Specify which named dataset to convert if multiple exist (for CSV)
        #[arg(long)]
        dataset: Option<String>,
    },
    /// Pack a human-readable .daxa file (with schema) into a binary .dax file
    Pack {
        /// Path to the input .daxa file
        #[arg(required = true)]
        input_path: PathBuf,
        /// Path for the output .dax file
        #[arg(required = true)]
        output_path: PathBuf,
        // TODO: Add options for compression, encryption
    },
    // TODO: `daxa schema generate <source_data_file>`
    // TODO: `daxa schema diff <schema1> <schema2>`
}


pub fn handle_command(cli: Cli) -> Result<String> {
    match cli.command {
        Commands::Info { file_path } => info::run_info(&file_path),
        Commands::Validate { file_path, schema_path } => validate::run_validate(&file_path, schema_path.as_deref()),
        Commands::Extract { file_path, data_path } => extract::run_extract(&file_path, &data_path),
        Commands::Convert { file_path, to, output, dataset } => convert::run_convert(&file_path, &to, output.as_deref(), dataset.as_deref()),
        Commands::Pack { input_path, output_path } => pack::run_pack(&input_path, &output_path),
    }
}