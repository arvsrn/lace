#[derive(Debug)]
pub struct Source {
    pub name:    String,
    pub content: String,
}

impl Source {
    pub fn new<S>(name: &S) -> Self
    where S: AsRef<std::ffi::OsStr> + ?Sized + std::fmt::Display {
        let file_path = std::path::Path::new(name);
        let buffer = std::fs::read_to_string(&file_path);
        let content = match buffer {
            Ok(content) => content,
            Err(message) => panic!("{}", message),
        };

        Self {
            name:    name.to_string(),
            content: content.to_string(),
        }
    }
}
