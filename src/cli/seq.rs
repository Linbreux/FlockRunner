use crate::project_config::{ProjectConfig};
use clap::Args;

#[derive(Debug, Args)]
#[command(about = "Run a sequence from the yaml file")]
pub struct SeqArgs {
}

pub fn handle_seq(_data: &SeqArgs, _project: &ProjectConfig){
}
