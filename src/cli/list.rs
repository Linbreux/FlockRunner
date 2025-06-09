use clap::Args;

use crate::yaml::project_config::ProjectConfig;

#[derive(Debug, Args)]
#[command(about = "List all possible commands/sequences")]
pub struct ListArgs {
}

pub fn handle_list(data: &ListArgs, project: &ProjectConfig){
    println!("Lising all possible commands");
    println!();
    println!("Commands");
    println!("========");
    for (command, value) in project.commands.iter(){
        println!("{}", command);
        if value.help.is_some() {
            println!("  {}", value.help.as_deref().unwrap());
        }
        println!("  cmd: {}", value.cmd);
        println!("");
    }
}
