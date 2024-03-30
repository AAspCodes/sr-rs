use log::SetLoggerError;
use simplelog::*;
use std::fs::File;

pub fn init_logger() -> Result<(), SetLoggerError>{
    WriteLogger::init(
        LevelFilter::Info, 
        Config::default(),    
        File::create("sr.log").unwrap(),
    )?;

    log::info!("Initializing logger");
    Ok(())
}
