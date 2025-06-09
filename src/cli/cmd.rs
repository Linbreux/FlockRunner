use clap::Args;
use crate::project_config::{ProjectConfig};

#[derive(Debug, Args)]
#[command(about = "Run a command from the yaml file")]
pub struct CmdArgs {
    #[arg(help = "the yaml command you want to run")]
    cmd: String,
}

pub fn handle_cmd(data: &CmdArgs, project: &ProjectConfig){
}
