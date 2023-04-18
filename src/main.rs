use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}

fn print_usage() {
    eprintln!("{} - replace all occurences of an input string with another", "quickreplacinator".green());
    eprintln!("Usage: quickreplacinator <search-regex> <replacement-string> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} - wrong number of arguments supplied. Expected 4, got {}.", "ERROR".red().bold(), args.len());
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();
    // println!("{:?}", args);

    let data = match fs::read_to_string(&args.filename) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} - failed to read from file {}: {}", "ERROR".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} - failed to replace text: {}", "ERROR".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} - failed to read from file {}: {}", "ERROR".red().bold(), args.output, e);
            std::process::exit(1);
        }
    };
}
