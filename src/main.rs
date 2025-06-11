// main.rs
mod yaml;
mod cli;

use crate::yaml::reader;
use crate::yaml::project_config;
use clap::Parser;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn get_final_config_path(config_input_path: &str) -> io::Result<PathBuf> {
    let path_as_obj = Path::new(config_input_path);

    if path_as_obj.is_absolute() {
        if fs::metadata(&path_as_obj).is_ok() {
            return Ok(path_as_obj.to_path_buf());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Absolute config file not found: '{}'", config_input_path),
            ));
        }
    }

    let current_working_dir = env::current_dir()?;

    let resolved_path_from_cwd = current_working_dir.join(path_as_obj);
    if fs::metadata(&resolved_path_from_cwd).is_ok() {
        return Ok(resolved_path_from_cwd);
    }
    let mut search_dir = current_working_dir.clone(); // Start search from CWD
    loop {
        let potential_config_path_upwards = search_dir.join(config_input_path);
        if fs::metadata(&potential_config_path_upwards).is_ok() {
            return Ok(potential_config_path_upwards);
        }

        if !search_dir.pop() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Could not find '{}' using direct resolution or by searching parent directories from the current working directory.",
                    config_input_path
                ),
            ));
        }
    }
}
fn main() {
    let mut yaml_reader = reader::Reader::new();
    let mut project = project_config::ProjectConfig::new();
    let mut yaml_file_path: std::path::PathBuf = Default::default();

    let cli = cli::base::Cli::parse();

    // Handle the --file flag
    match cli.file {
        Some(yaml_file) => {
            match get_final_config_path(yaml_file.to_str().unwrap()) {
                Ok(path) => yaml_file_path = path,
                Err(e) => eprintln!("Error: {}", e),
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
    cli::base::handle_command(&cli.command, &cli.vars, &mut project);
    

}
