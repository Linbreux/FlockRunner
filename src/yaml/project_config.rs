use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::yaml::reader::{Reader};

/// Represents the top-level structure of the YAML configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub project: String,
    // Variables are now directly a map of String to String
    pub variables: HashMap<String, String>,
    pub commands: HashMap<String, CommandDef>,
    pub sequence: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommandDef {
    pub cmd: String,
    pub alias: Option<String>, // Alias is optional
    pub help: Option<String>, // help text
    // r#type: Option<String>,
}

impl ProjectConfig{
    pub fn read(&mut self, reader: &Reader ){
        // Convert to yaml
        let yaml_content = serde_yaml::from_str(&reader.get_raw_content());
        match yaml_content{
            Ok(data) => *self = data,
            Err(err) => eprintln!("Could not parse yaml: {}", err),
        }
    }

    pub fn new() -> Self {
        ProjectConfig {
            project: String::new(),
            variables: HashMap::new(),
            commands: HashMap::new(),
            sequence: HashMap::new(),
        }
    }
}

