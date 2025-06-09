use std::fs;
use std::io;

pub struct Reader{
    file_path: Box<std::path::Path>,
    content: String
}

impl Reader{
    pub fn new() -> Self {
        Reader {
            file_path: std::path::PathBuf::new().into_boxed_path(),
            content: String::new(),
        }
    }

    pub fn load_file(&mut self, yaml_file_path: std::path::PathBuf) -> Result<(), io::Error> {
        self.file_path = yaml_file_path.into_boxed_path();

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
