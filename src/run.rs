use crate::config::Config;
use std::{fs, io::Write};

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(&config.source_file)?;
    let mut buffer = text.clone();
    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&config.macros[0].1)?;

    for (macro_name, replacement) in config.macros.iter() {
        buffer = buffer.replace(macro_name, replacement);
    }

    output_file.write(buffer.as_bytes())?;
    return Ok(());
}
