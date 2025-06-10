use crate::project_config::{ProjectConfig, CommandDef};
use std::collections::HashMap;
use std::process::{Command, Output};
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

pub fn parse_command(command: &CommandDef, variables: &HashMap<String, String>) -> String{
    let mut processed_command = command.cmd.clone();

    // Perform variable substitution
    // Looks for patterns like {{variable_name}} and replaces them with their value.
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key); // e.g., {{d}}

        let escaped_value = value.replace('\'', "'\\''");
        let quoted_value = format!("{}", escaped_value);

        // Replace the placeholder with the safely quoted value
        processed_command = processed_command.replace(&placeholder, &quoted_value);
    }

    return processed_command;
}

pub fn execute_shell_command(command: &CommandDef, variables: &HashMap<String, String>) -> Result<(), String> {

    let processed_command = parse_command(command, variables);
    // Use `sh -c` to allow executing arbitrary shell commands,
    // including pipes, redirections, etc. This is important for commands
    // like `date` or other shell features to be correctly interpreted.
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(&processed_command) // Use the processed command string after substitution
        .output()
        .map_err(|e| format!("Failed to execute command '{}': {}", processed_command, e))?;

    // Print stdout if available
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Print stderr if available
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // Check if the command exited successfully
    if !output.status.success() {
        return Err(format!(
            "Command '{}' exited with non-zero status: {:?}",
            processed_command, output.status
        ));
    }

    Ok(())
}

impl CommandArguments{

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
            _ => ()
        }
    }

    pub fn search_command(&self, project: &ProjectConfig) -> Option<CommandDef>{
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
