use std::fs::{self, File};
use std::io::{BufWriter, Write};

use shared::data_types::BYTE;

const PATH: &str = "Kalopsia-Steps/data";

/// Appends data into a specified file.
///
/// Creates the specified file if it doesn't exist and truncates if it does.
///
/// # Panics
/// Panics if the path doesn't exist.
pub fn appendf(data: &[u8], path: &str) -> std::io::Result<()> {
    let file = File::create(path);
    let mut buf_writer = BufWriter::new(file.unwrap());

    match buf_writer.write_all(data) {
        Ok(()) => {
            buf_writer.flush()?;
            Ok(())
        }
        Err(why) => {
            buf_writer.flush()?;
            panic!("[ERROR] could not append to file.\nreason: '{}'", why)
        }
    }
}

/// Reads data into a `Vec<BYTE>`, where `BYTE` is an `u8`
///
/// # Panics
/// Panics if the file does not exist or isn't found.
pub fn readf(path: &str) -> Vec<BYTE> {
    let content = fs::read(path);

    match content {
        Ok(_) => content.unwrap(),
        Err(why) => panic!("[ERROR] could not read file.\nreason: '{}'", why),
    }
}

/// Deletes a specified file.
///
/// # Panics
/// Panics if the file to be deleted does not exist or isn't found.
pub fn deletef<T>(_data: T, file: &str) {
    match std::fs::remove_file(format!("{}/{}", PATH, file)) {
        Ok(_) => println!("[INFO] deleted file '{}'", file),
        Err(why) => panic!("[ERROR] could not delete file '{}': {}", file, why),
    }
}

// TODO: add remove data from file function
