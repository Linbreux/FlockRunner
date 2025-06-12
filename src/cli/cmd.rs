use clap::Args;
use crate::project_config::{ProjectConfig};
use crate::project_config::CommandDef;
use crate::cli::command_handler;
use crate::yaml::project_config::CommandValue;
use colored::Colorize;

#[derive(Debug, Args)]
#[command(about = "Run a command from the yaml file")]
pub struct CmdArgs {
    #[arg(help = "the yaml command you want to run")]
    cmd: String,


    #[arg(long = "dryrun")]
    #[arg(help = "Show all commands without running them")]
    pub dryrun: bool,

    #[arg(short, long)]
    #[arg(help = "Add more information about the commands that will run")]
    verbose: bool,
}

pub fn search_command(
    data: &CmdArgs,
    project: &ProjectConfig
) -> Option<CommandDef>{
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

fn process_single_command (
    command: &CommandDef,
    exec_command: &String,
    data: &CmdArgs,
    project: &ProjectConfig, // Assuming `project` is ProjectConfig
) -> Result<(), String> {
    if data.verbose || data.dryrun {
        println!("{}", format!("== Running {} command ==", data.cmd).blue());
    }
    if data.dryrun {
        // Do not run the command. Just show it.
        println!("dry: {}", format!("{} {}", 
            command_handler::return_shell(&project, command.shell.as_ref()),
            command_handler::parse_command(&exec_command, &project.variables)).bold()
        );
    } else {
        return command_handler::execute_shell_command(&command, &exec_command, &project);
    }
    Ok(())
}

pub fn handle_cmd(
    data: &CmdArgs,
    project: &ProjectConfig
){
    let search_opt = search_command( data, &project);

    if let Some(command) = search_opt{
        match command.cmd {
            CommandValue::String(ref exec_command) => {
                let result = process_single_command(&command, &exec_command, data, project);
                match result{
                    Ok(_) => if data.verbose {
                            println!("{}", format!("Succesfully ran!").green())
                    },
                    Err(err) => println!("{}",format!("Command failed!").red())
                }
            }
            CommandValue::List(ref l) => {
                for exec_command in l {
                    let result = process_single_command(&command, &exec_command, data, project);

                    match result{
                        Ok(_) => if data.verbose {
                            println!("{}", format!("Succesfully ran!").green())
                        },
                        Err(err) => {
                            println!("{}",format!("Command failed!").red());
                            if let Some(kg) = command.keep_going{
                                println!("{}", format!("Keep going flag enabled. Continuing!").yellow());
                            }else{
                                break;
                            }
                        }
                    }
                }
            }
        }
    }else{
        eprintln!("Could not run command {}", data.cmd);
        return;
    }
}
