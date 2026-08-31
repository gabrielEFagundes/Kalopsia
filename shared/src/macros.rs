#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        print!("[DEBUG] {}\n", format_args!($($arg)*));
    };
}
