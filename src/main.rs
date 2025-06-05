mod config;
mod run;

use config::Config;
use run::run;

fn main() {
    match Config::new() {
        Ok(config) => {
            if let Err(e) = run(&config) {
                eprintln!("Runtime error: {e}")
            }
        }
        Err(e) => eprintln!("Parsing error: {e}"),
    }
}
