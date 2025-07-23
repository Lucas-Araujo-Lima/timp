use std::{collections::HashMap, str::FromStr};

pub struct Config {
    pub source_file: String,
    pub output_file: String,
    pub macros: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        // arg parsing
        let mut config = Config {
            source_file: String::new(),
            output_file: String::new(),
            macros: HashMap::new(),
        };

        let mut args = std::env::args();
        args.next();

        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let source_name = args.next();
        if let Some(name) = source_name {
            config.source_file = home + "/Templates/" + &name;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Please give a template source filename",
            )));
        }

        let output_filename = args.next();
        if let Some(filename) = output_filename {
            config.output_file = filename.clone();
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Please give an output filename",
            )));
        }

        for arg in args {
            if arg.starts_with("-") {
                let equal_index = arg.find("=");
                if let Some(index) = equal_index {
                    let macro_name = String::from_str(&arg[1..index].trim()).unwrap();
                    let definition = String::from_str(&arg[index + 1..].trim()).unwrap();
                    if macro_name.len() == 0 {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Bad macro. No macro name given",
                        )));
                    }
                    if definition.len() == 0 {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Bad macro. No definition given",
                        )));
                    }
                    config.macros.insert(macro_name, definition);
                } else {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Bad macro. No '=' character",
                    )));
                }
            }
        }
        return Ok(config);
    }
}
