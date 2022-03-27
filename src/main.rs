pub mod cargoreader;
pub mod common;
pub mod utils;

use std::env;
// use utils::Result;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: carp <command>");
        return;
    }
    match args[1].to_lowercase().as_str() {
        "test" => match cargoreader::read_cargo_file() {
            Ok(file) => match cargoreader::parse_cargo_file(file) {
                Ok(parsed) => println!("Ok: {:?}", parsed),
                Err(err) => eprintln!("Parse error: {}", err),
            },
            Err(err) => eprintln!("Read error: {}", err),
        },
        unknown_command => eprintln!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown_command
        ),
    }
}
