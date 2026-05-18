#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if std::env::var("GITX_DEBUG").is_ok() {
            eprintln!(
                "[{}] [DEBUG] {}",
                ::chrono::Local::now().format("%H:%M:%S.%3f"),
                format!($($arg)*)
            );
        }
    }
}
