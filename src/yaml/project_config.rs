use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::yaml::reader::{Reader};

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    #[allow(dead_code)]
    pub project: String,

    // Variables are now directly a map of String to String
    pub variables: HashMap<String, String>,
    pub commands: HashMap<String, CommandDef>,

    #[allow(dead_code)]
    pub sequence: Option<HashMap<String, Vec<String>>>,
    pub shells: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommandDef {
    pub cmd: CommandValue,
    pub alias: Option<String>, // Alias is optional
    pub help: Option<String>, // help text
    pub keep_going: Option<bool>,
    pub shell: Option<String>,
    pub variables: Option<HashMap<String, String>>,
    // r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CommandValue {
    List(Vec<String>),
    String(String),
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
            sequence: None,
            shells: None,
        }
    }
}

