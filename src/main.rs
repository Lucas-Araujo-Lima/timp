use std::str::FromStr;

pub struct Options {
    source_file: String,
    macros: Vec<(String, String)>,
}

fn main() {
    // arg parsing
    let mut option = Options {
        source_file: String::new(),
        macros: Vec::new(),
    };

    let mut args = std::env::args();
    args.next();

    let home = std::env::var("HOME").expect("HOME environment variable not set");
    option.source_file = home
        + "/Templates/"
        + &args
            .next()
            .expect("Error: Please give a template source filename");

    let output_filename = args.next();
    if let Some(filename) = output_filename {
        option.macros.push((
            String::from_str("filename").unwrap(),
            String::from_str(&filename).unwrap(),
        ))
    } else {
        eprintln!("Error: Please give a output filename");
        return;
    }

    for arg in args {
        if arg.starts_with("-") {
            let equal_index = arg.find("=");
            if let Some(index) = equal_index {
                let macro_name = String::from_str(&arg[1..index].trim()).unwrap();
                let replacement = String::from_str(&arg[index + 1..].trim()).unwrap();
                if macro_name == "filename" {
                    eprintln!(
                        "Error: filename macro is reserved. It is set automatically as the output filename"
                    );
                    return;
                }
                option.macros.push((macro_name, replacement));
            } else {
                eprintln!("Usage: -macro=replacement");
            }
        }
    }

    println!("{}", option.source_file);
    for (macro_name, replacement) in option.macros {
        println!("{macro_name}    {replacement}");
    }
}
