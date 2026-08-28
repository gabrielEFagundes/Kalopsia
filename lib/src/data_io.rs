use std::fs::File;
use std::io::{BufWriter, Write};

const PATH: &str = "Kalopsia-Steps/data";

/// Appends data into a specified file.
///
/// Creates the specified file if it doesn't exist.
pub fn appendf(data: &[u8], path: &str) -> std::io::Result<()> {
    println!("[DEBUG] entered appendf");

    let file = File::create(path);
    let mut buf_writer = BufWriter::new(file.unwrap());

    match buf_writer.write_all(data) {
        Ok(()) => {
            buf_writer.flush()?;
            println!("match OK");
            Ok(())
        },
        Err(why) => {
            buf_writer.flush()?;
            panic!("[ERROR] could not append to file.\nreason: '{}'", why)
        }
    }
}

/// Deletes a specified file.
pub fn deletef<T>(_data: T, file: &str) {
    match std::fs::remove_file(format!("{}/{}", PATH, file)) {
        Ok(_) => println!("[INFO] deleted file '{}'", file),
        Err(why) => panic!("[ERROR] could not delete file '{}': {}", file, why),
    }
}

// TODO: add remove data from file function