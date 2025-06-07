// main.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::{Command, Output};

/// Represents the top-level structure of the YAML configuration.
#[derive(Debug, Deserialize, Serialize)]
struct ProjectConfig {
    project: String,
    // Variables are now directly a map of String to String
    variables: HashMap<String, String>,
    commands: HashMap<String, CommandDef>,
    sequence: HashMap<String, Vec<String>>,
}

/// Represents a single command definition, including its command string and alias.
/// Added `Deserialize` and `Serialize` derives for serde to work correctly.
/// If you uncommented 'type', it would need to be r#type
#[derive(Debug, Deserialize, Serialize)]
struct CommandDef {
    cmd: String,
    alias: Option<String>, // Alias is optional
    // r#type: Option<String>,
}

/// Executes a given shell command after performing variable substitution and prints its output.
///
/// Arguments:
/// * `command_str`: The raw string representing the shell command with potential placeholders.
/// * `variables`: A HashMap containing key-value pairs for variable substitution.
///
/// Returns:
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`).
fn execute_shell_command(command_str: &str, variables: &HashMap<String, String>) -> Result<(), String> {
    let mut processed_command = command_str.to_string();

    // Perform variable substitution
    // Looks for patterns like {{variable_name}} and replaces them with their value.
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key); // e.g., {{d}}
        processed_command = processed_command.replace(&placeholder, value);
    }

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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    // Separate command-line arguments into non-variable arguments and CLI variable assignments.
    // Example: `flokerunner config.yaml cmd greet_date -v my_var=some_value`
    let mut cli_variables: HashMap<String, String> = HashMap::new();
    let mut non_var_args: Vec<String> = Vec::new();

    // Skip the program name (args[0]) during parsing.
    for arg in args.into_iter().skip(1) {
        if arg.contains('=') {
            let parts: Vec<&str> = arg.splitn(2, '=').collect();
            if parts.len() == 2 {
                cli_variables.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                // If an argument contains '=', but isn't a valid key=value pair (e.g., just '='),
                // treat it as a non-variable argument.
                non_var_args.push(arg);
            }
        } else {
            non_var_args.push(arg);
        }
    }

    // Basic argument validation: Ensure we have at least `yaml_file_path`, `subcommand`, and `name`.
    if non_var_args.len() < 3 {
        eprintln!("Usage: {} <yaml_file_path> <cmd|seq> <name> [-v] [variable=value ...]", env::args().next().unwrap_or_else(|| "flokerunner".to_string()));
        return Err("Not enough arguments provided.".to_string());
    }

    let yaml_file_path = &non_var_args[0];
    let subcommand = &non_var_args[1];
    let name = &non_var_args[2];
    // Check for the verbose flag among the non-variable arguments.
    let verbose = non_var_args.iter().any(|arg| arg == "-v");

    // Read the YAML configuration file.
    let yaml_content = fs::read_to_string(yaml_file_path)
        .map_err(|e| format!("Failed to read YAML file '{}': {}", yaml_file_path, e))?;

    // Parse the YAML content into the `ProjectConfig` struct.
    let config: ProjectConfig = serde_yaml::from_str(&yaml_content)
        .map_err(|e| format!("Failed to parse YAML content: {}", e))?;

    // Create a merged set of variables:
    // Start with variables from the YAML file, then override with any variables
    // provided via the command line. CLI variables take precedence.
    let mut merged_variables = config.variables;
    for (key, value) in cli_variables {
        merged_variables.insert(key, value);
    }

    // Handle `cmd` or `seq` subcommand execution.
    match subcommand.as_str() {
        "cmd" => {
            // Find the specified command by its name or alias.
            let mut found_cmd_def: Option<&CommandDef> = None;
            for (cmd_name, cmd_def) in &config.commands {
                if cmd_name == name {
                    found_cmd_def = Some(cmd_def);
                    break;
                }
                if let Some(alias) = &cmd_def.alias {
                    if alias == name {
                        found_cmd_def = Some(cmd_def);
                        break;
                    }
                }
            }

            if let Some(cmd_def) = found_cmd_def {
                if verbose {
                    println!("running {} command", name);
                }
                // Execute the command, passing the merged variables for substitution.
                execute_shell_command(&cmd_def.cmd, &merged_variables)?;
            } else {
                return Err(format!("Command or alias '{}' not found.", name));
            }
        }
        "seq" => {
            // Find the specified sequence.
            if let Some(sequence_commands) = config.sequence.get(name) {
                println!("Running sequence '{}':", name);
                for cmd_name_in_seq in sequence_commands {
                    // For each command in the sequence, find its definition.
                    let mut found_cmd_def: Option<&CommandDef> = None;
                    for (actual_cmd_name, cmd_def) in &config.commands {
                        if actual_cmd_name == cmd_name_in_seq {
                            found_cmd_def = Some(cmd_def);
                            break;
                        }
                    }

                    if let Some(cmd_def) = found_cmd_def {
                        if verbose {
                            println!("  Executing command '{}' from sequence...", cmd_name_in_seq);
                        }
                        // Execute the command from the sequence, with variable substitution.
                        execute_shell_command(&cmd_def.cmd, &merged_variables)?;
                    } else {
                        eprintln!("Warning: Command '{}' in sequence '{}' not found. Skipping.", cmd_name_in_seq, name);
                    }
                }
            } else {
                return Err(format!("Sequence '{}' not found.", name));
            }
        }
        _ => {
            // Handle unknown subcommands.
            return Err(format!("Unknown subcommand '{}'. Use 'cmd' or 'seq'.", subcommand));
        }
    }

    Ok(())
}

