use crate::config::Config;
use std::{collections::HashMap, fs, io::Write};

const SETTINGS_START_SYMBOL: &str = "TIMPSTART";
const SETTINGS_END_SYMBOL  : &str = "TIMPEND";

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(&config.source_file)?;

    // grabs macro definitions in template settings
    let mut template_settings: HashMap<String, String> = HashMap::new();
    let mut template_settings_text = text.lines().skip_while(|s| *s != SETTINGS_START_SYMBOL).into_iter();
    if let Some(_start_symbol) = template_settings_text.next() {
        for setting in template_settings_text {
            let mut words = setting.split('=');
            let macro_name = words.next().expect("No macro name given in template settings. Perhaps an empty line?");
            if macro_name == SETTINGS_END_SYMBOL {
                break;
            }
            let definition = words.next().expect(&format!("No macro definition given in template settings for macro {macro_name}"));
            template_settings.insert(macro_name.to_string(), definition.to_string());
        }
    }
    
    let mut buffer: String = text.lines().skip_while(|s| *s != SETTINGS_END_SYMBOL).skip(1).map(|l| format!("{l}\n")).collect();
    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&config.output_file)?;

    // applies user given definitions
    for (macro_name, definition) in config.macros.iter() {
        buffer = buffer.replace(macro_name, definition);
    }

    // applies definitions from template settings
    for (macro_name, definition) in template_settings.iter() {
        buffer = buffer.replace(macro_name, definition);
    }

    output_file.write(buffer.as_bytes())?;
    return Ok(());
}
