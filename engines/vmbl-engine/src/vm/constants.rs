use crate::vm::interpreter::Interpreter;

/// Reads UTF-8 characters considering the byte before them, which is the string length,
/// or the amount to read.
pub fn interpret_string_byte(interpreter: &mut Interpreter, length: u8) -> String {
    let mut mounted_str: Vec<char> = Vec::new();

    for _i in 0..length {
        interpreter.read();
        mounted_str.push(interpreter.bytes[interpreter.cursor as usize] as char);
    }

    mounted_str.into_iter().collect()
}

/// Reads 4 bytes (or a signed 32-bit integer)
pub fn interpret_int_byte(interpreter: &mut Interpreter) -> i32 {
    let mut bytes: Vec<u8> = Vec::new();

    for _i in 0..=3 {
        bytes.push(interpreter.bytes[interpreter.cursor as usize]);
        interpreter.read();
    }

    i32::from_le_bytes(
        bytes
            .try_into()
            .expect("[ERROR] 32-bit integer does not contain 4 bytes"),
    )
}

/// Reads 8 bytes (or a signed 64-bit floating point)
pub fn interpret_double_byte(interpreter: &mut Interpreter) -> f64 {
    let mut bytes: Vec<u8> = Vec::new();

    for _i in 0..=7 {
        bytes.push(interpreter.bytes[interpreter.cursor as usize]);
        interpreter.read();
    }

    f64::from_le_bytes(
        bytes
            .try_into()
            .expect("[ERROR] 64-bit floating point does not contain 8 bytes"),
    )
}
