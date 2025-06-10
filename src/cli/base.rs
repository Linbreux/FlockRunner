use std::env;
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use crate::{cli::{cmd, list, seq}, yaml::project_config::ProjectConfig};

#[derive(Parser)]
#[command(name = "FlockRunner")]
#[command(author = "Linbreux <linbreux@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Yaml command executor", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long = "var", value_parser = parse_key_value)]
    pub vars: Vec<(String, String)>,

    #[arg(long, default_value = "flockrunner.yaml")]
    pub file: Option<std::path::PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Cmd(cmd::CmdArgs),
    Seq(seq::SeqArgs),
    List(list::ListArgs),
}

pub fn handle_command(command: &Commands, project: &ProjectConfig) {
    match &command {
        Commands::List(data) => {
            list::handle_list(&data, &project);
        }
        Commands::Cmd(data) => {
            cmd::handle_cmd(&data, &project);
        }
        Commands::Seq(data) => {
            seq::handle_seq(&data, &project);
        }
    }
}

pub fn command_parser(
    cli_variables: &mut HashMap<String, String>,
    non_var_args: &mut Vec<String>
){
    let args: Vec<String> = env::args().collect();

    for arg in args.into_iter().skip(1) {
        if arg.contains('=') {
            let parts: Vec<&str> = arg.splitn(2, '=').collect();
            if parts.len() == 2 {
                cli_variables.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                // if an argument contains '=', but isn't a valid key=value pair (e.g., just '='),
                // treat it as a non-variable argument.
                non_var_args.push(arg);
            }
        } else {
            non_var_args.push(arg);
        }
    }
}

// returns the location of the yaml if all ok
pub fn check_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let mut usefull_args: Vec<String>=vec![];

    for arg in args.into_iter().skip(1) {
        if ! arg.contains('=') {
            usefull_args.push(arg);
        }
    }

    if usefull_args.len() < 3 {
        eprintln!("Usage: {} <yaml_file_path> <cmd|seq|list> <name> [-v] [variable=value ...]", env::args().next().unwrap_or_else(|| "flockrunner".to_string()));
        return Err("Not enough arguments...".to_string())
    }
    Ok(usefull_args[0].clone())
}
