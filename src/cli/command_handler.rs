use crate::project_config::{ProjectConfig, CommandDef};
use std::collections::HashMap;
use crate::cli::base;

enum SubCommand {
    Cmd,
    Seq,
    List,
    None
}

pub struct CommandArguments{
    file_location: String,
    command: SubCommand,
    command_name: String,
    verbose: bool,

    variables: HashMap<String, String>,
}

impl CommandArguments{
    pub fn create(project: &ProjectConfig) ->Self
    {
        // cli arguments
        let mut cli_variables: HashMap<String, String> = HashMap::new();
        let mut usefull_args: Vec<String> = Vec::new();

        base::command_parser(&mut cli_variables, &mut usefull_args);

        let mut merged_variables = project.variables.clone();
        for (key, value) in cli_variables {
            merged_variables.insert(key.to_string(), value.to_string());
        }

        CommandArguments{
            file_location: usefull_args[0].clone(),
            command: CommandArguments::string_to_subcommand(&usefull_args[1]),
            command_name: usefull_args[2].clone(),
            verbose: usefull_args.iter().any(|arg| arg == "-v"),
            variables: merged_variables
        }
    }

    fn string_to_subcommand(s: &String)->SubCommand
    {
        match s.as_str(){
            "cmd" => SubCommand::Cmd,
            "seq" => SubCommand::Seq,
            "list" => SubCommand::List,
            _ => SubCommand::None
        }
    }

    pub fn subcommand_handler(
        &self,
        project: &ProjectConfig,
    ) {
        match self.command{
            SubCommand::Cmd=>{
                let search_opt = self.search_command(&project);
                if let Some(_command) = search_opt{
                    if self.verbose{
                        println!("Running {} command", self.command_name)
                    }
                }else{
                    eprintln!("Could not run command {}", self.command_name);
                    return;
                }

            },
            SubCommand::Seq=>{},
            SubCommand::List=>{
                println!("Lising all possible commands");
                println!();
                for (command, value) in project.commands.iter(){
                    println!("{} {}", command, value.help.as_deref().unwrap_or(""));
                }
            }
            _ => ()
        }
    }

    fn search_command(&self, project: &ProjectConfig) -> Option<CommandDef>{
        // Find the specified command by its name or alias.
        let mut found_cmd_def: Option<CommandDef> = None;
        for (cmd_name, cmd_def) in &project.commands {
            if *cmd_name == self.command_name {
                found_cmd_def = Some(cmd_def.clone());
                break;
            }
            if let Some(alias) = &cmd_def.alias {
                if *alias == self.command_name {
                    found_cmd_def = Some(cmd_def.clone());
                    break;
                }
            }
        }
        return found_cmd_def;
    }
}
