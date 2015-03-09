use std::fs::File;
use std::io::Write;

pub struct FileIo {
    filename: String,
    file: Option<File>,
}

impl FileIo {
    pub fn new(name: String) -> FileIo {
        FileIo {
            filename: name.clone(),
            file: match File::create(&Path::new(name.clone())) {
                Ok(f) => Some(f),
                Err(e) => if name == "" {
                    None
                } else {
                    panic!("Could not open file:  {}", e);
                }
            },
        }
    }

    pub fn write(&mut self, message: &str) -> bool {
        if self.filename == "" {
            println!("{}", message);
            true
        } else {
            match self.file.as_mut().unwrap().write(message.as_bytes()) {
                Ok(_) => true,
                Err(e) => {
                    println!("Cannot write:  {}", e);
                    false
                },
            }
        }
    }
}

