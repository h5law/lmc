use std::fmt;

// LogLevel defines the different log levels
pub enum LogLevel {
    Info,
    Debug,
    Error,
}

// Implement the Display trait for LogLevel.
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

// Logger is a simple logger that can be used to print messages to the console.
pub struct Logger {
    verbose: bool,
    debug: bool,
}

impl Logger {
    // Create a new Logger.
    pub fn new(verbose: bool, debug: bool) -> Self {
        Logger { verbose, debug }
    }

    // Log a message at the given level.
    pub fn log(&self, level: &LogLevel, message: &str) {
        match level {
            LogLevel::Info => {
                if self.verbose {
                    println!("{}: {}", level, message)
                }
            }
            LogLevel::Debug => {
                if self.debug {
                    println!("{}: {}", level, message)
                }
            }
            LogLevel::Error => println!("{}: {}", level, message),
        }
    }
}
