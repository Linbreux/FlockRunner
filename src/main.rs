// main.rs
mod yaml;
mod cli;

use crate::yaml::reader;
use crate::yaml::project_config;

fn main() {
    let mut yaml_reader = reader::Reader::new();
    let mut project = project_config::ProjectConfig::new();
    let mut yaml_file: String;

    let result_check = cli::base::check_args();
    match result_check{
        Ok(yaml_file_path) => yaml_file = yaml_file_path,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    }
    let load_result = yaml_reader.load_file(&yaml_file);

    match load_result {
        Ok(_) => project.read(&yaml_reader),
        Err(e) => println!("{}", e.to_string())
    }
    let cli_handler = cli::command_handler::CommandArguments::create(&project);

    cli_handler.subcommand_handler(&project);
}
