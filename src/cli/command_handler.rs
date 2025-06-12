use std::collections::HashMap;
use std::process::{Command, Output};

use crate::yaml::project_config::{CommandDef, ProjectConfig};

pub fn parse_command(command: &String, variables: &HashMap<String, String>) -> String{
    let mut processed_command = command.clone();

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

pub fn return_shell(project: &ProjectConfig, shell: Option<&String>) -> String{
    let default_shell: String = "sh -c".to_string();
    match shell{
        Some(s) => {
            project.shell
                .as_ref()
                .and_then(|shells| shells.get(s))
                .map(|shell_command| shell_command)
                .unwrap_or(&default_shell).to_string()
        }
        None=> default_shell.to_string()
    }
}

pub fn execute_shell_command(command: &CommandDef, exec_command: &String, project: &ProjectConfig) -> Result<(), String> {

    let processed_command = parse_command(&exec_command, &project.variables);

    let shell = parse_command(&return_shell(&project, command.shell.as_ref()), &project.variables);

    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} \"{}\"", &shell, &processed_command))
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
