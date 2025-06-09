use std::fs;
use std::io;

pub struct Reader{
    file_path: String,
    content: String
}

impl Reader{
    pub fn new() -> Self {
        Reader {
            file_path: String::new(),
            content: String::new(),
        }
    }

    pub fn load_file(&mut self, yaml_file_path: &str) -> Result<(), io::Error> {
        self.file_path = yaml_file_path.to_string();

        let file_content = fs::read_to_string(&self.file_path);
        match file_content{
            Ok(content) => {
                self.content = content;
                Ok(())
            }
            Err(err) => Err(err)
        }
    }

    pub fn get_raw_content(&self) -> &str {
        &self.content
    }
}
