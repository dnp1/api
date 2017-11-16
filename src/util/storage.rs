use std::io::{Write, BufWriter, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::error;
use std::result;
use std::io::BufReader;

pub trait Storage: Send + Sync + 'static {
    fn save<I>(&self, filename: &str, content: &mut I) -> StorageResult<usize> where I : BufRead + Send;
    fn retrieve (&self, filename: &str) -> StorageResult<Box<BufRead + Send>>;
}


type StorageResult<T> = result::Result<T, StorageError>;

#[derive(Debug)]
pub struct StorageError {}
impl error::Error for StorageError{
    fn description(&self) -> &str {
        "Could not save file"
    }
}

use std::fmt;
impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

pub struct DiskStorage {
    directory: PathBuf
}

impl DiskStorage {
    pub fn new(directory: &str) -> DiskStorage {
        DiskStorage{directory: PathBuf::from(directory)}
    }
}

impl Storage for DiskStorage {
    fn save<I>(&self, filename: &str, content:&mut I) -> StorageResult<usize>   where I : BufRead + Send {
        let file_path = self.directory.join(filename);
        let file = match File::create(file_path) {
            Err(_) => return Err(StorageError{}),
            Ok(file) => file,

        };
        let mut written: usize = 0;
        let mut buffer = BufWriter::new(file);
        loop {
            let read = match content.fill_buf() {
                Err(err) => return Err(StorageError{}),
                Ok(bytes) => match buffer.write_all(bytes) {
                    Err(err) => return Err(StorageError{}),
                    Ok(_) => {
                        written += bytes.len();
                        bytes.len()
                    },
                },
            };
            if read == 0 {
                break;
            }
            content.consume(read);
        }
        match buffer.flush() {
            Err(_) => Err(StorageError{}),
            Ok(_) => Ok(written),
        }

    }
    fn retrieve(&self, filename: &str) -> StorageResult<Box<BufRead + Send>> {
        let file_path = self.directory.join(filename);
        let file = match File::open(file_path) {
            Err(_) => return Err(StorageError{}),
            Ok(file) => file,

        };
        Ok(Box::new(BufReader::with_capacity(32768, file)))
    }
}