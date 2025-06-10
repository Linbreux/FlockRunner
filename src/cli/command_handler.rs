use std::collections::HashMap;
use std::process::{Command, Output};

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

pub fn execute_shell_command(command: &String, variables: &HashMap<String, String>) -> Result<(), String> {

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
