use clap::Args;
use crate::project_config::{ProjectConfig};
use crate::project_config::CommandDef;
use crate::cli::command_handler::execute_shell_command;

#[derive(Debug, Args)]
#[command(about = "Run a command from the yaml file")]
pub struct CmdArgs {
    #[arg(help = "the yaml command you want to run")]
    cmd: String,

    #[arg(short, long)]
    #[arg(help = "Add more information about the commands that will run")]
    verbose: bool,
}


pub fn search_command(data: &CmdArgs, project: &ProjectConfig) -> Option<CommandDef>{
    // Find the specified command by its name or alias.
    let mut found_cmd_def: Option<CommandDef> = None;
    for (cmd_name, cmd_def) in &project.commands {
        if *cmd_name == data.cmd {
            found_cmd_def = Some(cmd_def.clone());
            break;
        }
        if let Some(alias) = &cmd_def.alias {
            if *alias == data.cmd {
                found_cmd_def = Some(cmd_def.clone());
                break;
            }
        }
    }
    return found_cmd_def;
}

pub fn handle_cmd(data: &CmdArgs, project: &ProjectConfig){
    let search_opt = search_command( data, &project);
    if let Some(command) = search_opt{
        if data.verbose{
            println!("Running {} command", data.cmd)
        }
        execute_shell_command(command, variables)

    }else{
        eprintln!("Could not run command {}", data.cmd);
        return;
    }
}
