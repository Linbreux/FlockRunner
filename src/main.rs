// main.rs
mod yaml;
mod cli;

use crate::yaml::reader;
use crate::yaml::project_config;
use clap::Parser;

fn main() {
    let mut yaml_reader = reader::Reader::new();
    let mut project = project_config::ProjectConfig::new();
    let mut yaml_file_path: std::path::PathBuf;

    let cli = cli::base::Cli::parse();

    // Handle the --file flag
    match cli.file {
        Some(yaml_file) => {
            yaml_file_path = yaml_file.clone();
            if !yaml_file.exists() {
                eprintln!("Error: The specified file does not exist: {:?}", yaml_file);
                std::process::exit(1);
            }
            if !yaml_file.is_file() {
                eprintln!("Error: The specified path is not a file: {:?}", yaml_file);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Error: The --file flag is required.");
            // Or provide a default behavior, show help, etc.
            // For example: cli::base::Cli::command().print_help().unwrap();
            std::process::exit(1);
        }
    }
    let load_result = yaml_reader.load_file(yaml_file_path);

    match load_result {
        Ok(_) => project.read(&yaml_reader),
        Err(e) => println!("{}", e.to_string())
    }
    cli::base::handle_command(&cli.command, &project);
    

}
