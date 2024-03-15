use simplelog::*;
use std::fs::File;

pub fn init_logger() {
    // Initialize WriteLogger to log to a file
    WriteLogger::init(
        LevelFilter::Info,               // Set the log level (e.g., Info, Warn, Error)
        Config::default(),               // Use default configuration
        File::create("sr.log").unwrap(), // Specify the log file
    )
    .unwrap();

    // Example log messages
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");
}
