use std::fs::File;
use std::io::Write;

const PATH: &str = "Kalopsia-Steps/data";

/// Appends data into a specified file.
///
/// Creates the specified file if it doesn't exist.
/// ### Parameters:
/// - `data: T` -> Generic data, specifically used for `Dev`, `Node` and `Edge` cases.
/// - `file: &str` -> The name of the file to be appended into.
pub fn appendf<T: std::fmt::Display>(data: T, file: &str) {
    File::create(format!("{}/{}", PATH, file)).unwrap();

    match File::options()
        .append(true)
        .open(format!("{}/{}", PATH, file))
        .and_then(|mut f| writeln!(f, "{}", data))
    {
        Ok(_) => println!("[INFO] Appended to file '{}/{}'", PATH, file),
        Err(_) => panic!("[ERROR] Could not append to file '{:#?}'", file),
    }
}

/// Deletes data from a specified file.
///
/// ### Parameters:
/// - `data: T` -> Generic data, specifically used for `Dev`, `Node` and `Edge` cases.
/// - `file: &str` -> The name of the file to be deleted.
pub fn deletef<T>(_data: T, file: &str) {
    match std::fs::remove_file(format!("{}/{}", PATH, file)) {
        Ok(_) => println!("[INFO] Deleted file '{}'", file),
        Err(why) => panic!("[ERROR] Could not delete file '{}': {}", file, why),
    }
}

// TODO: add remove data from file function