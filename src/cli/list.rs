use clap::Args;

use crate::yaml::project_config::ProjectConfig;
use crate::yaml::project_config::CommandValue;

#[derive(Debug, Args)]
#[command(about = "List all possible commands/sequences")]
pub struct ListArgs {
}

pub fn handle_list(_data: &ListArgs, project: &ProjectConfig){
    println!("Lising all possible commands");
    println!();
    println!("Commands");
    println!("========");
    for (command, value) in project.commands.iter(){
        println!("{}", command);
        if value.help.is_some() {
            println!("  {}", value.help.as_deref().unwrap());
        }

        match &value.cmd{
            CommandValue::String(exec_command) => {
                println!("  cmd: {}", exec_command);
            }
            CommandValue::List(exec_list) => {
                for exec_command in exec_list{
                    println!("  cmd: {}", exec_command);
                }
            }
        }
        println!("");
    }
}
