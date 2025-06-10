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

    #[arg(long = "var",short = 'D', global=true, value_parser = parse_key_value)]
    #[arg(help = "Override a project variable. ex. var1=hoi")]
    pub vars: Vec<(String, String)>,

    #[arg(long,default_value = "flockrunner.yaml")]
    pub file: Option<std::path::PathBuf>,
}

// Custom value parser for key-value pairs
fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() == 2 {
        Ok((parts[0].to_string(), parts[1].to_string()))
    } else {
        Err(format!("Invalid key-value pair: {}", s))
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Cmd(cmd::CmdArgs),
    Seq(seq::SeqArgs),
    List(list::ListArgs),
}

pub fn handle_command(
    command: &Commands,
    vars: &Vec<(String, String)>,
    project: &mut ProjectConfig
) {
    let variables: HashMap<String, String> = vars.into_iter().cloned().collect();
    // merge cli vars with project once. cli has priority.
    project.variables.extend(variables);

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
