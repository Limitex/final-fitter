#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        println!("{} {}", ::console::style("✓").green(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{} {}", ::console::style("●").green(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_dim {
    ($($arg:tt)*) => {
        println!("{} {}", ::console::style("○").dim(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("{} {}", ::console::style("!").yellow(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", ::console::style("error:").red().bold(), format_args!($($arg)*))
    };
}
