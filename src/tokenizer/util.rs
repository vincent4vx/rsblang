use std::error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::tokenizer::Tokenizer;

impl Tokenizer {
    pub fn from_file(file: &Path) -> Result<Tokenizer, Box<dyn error::Error>> {
        let mut file = match File::open(file) {
            Ok(file) => file,
            Err(e) => return Err(e.into())
        };

        let mut buf: [u8; 256] = [0; 256];
        let mut tokenizer = Tokenizer::new();

        loop {
            let size = match file.read(buf.as_mut()) {
                Ok(size) => size,
                Err(e) => return Err(e.into())
            };

            for i in 0..size {
                match tokenizer.push(char::from(buf[i])) {
                    Ok(_) => {}
                    Err(e) => return Err(e.into())
                }
            }

            if size < 256 {
                break
            }
        }

        match tokenizer.finalize() {
            Ok(_) => Ok(tokenizer),
            Err(e) => Err(e.into())
        }
    }
}
