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
        "test" => println!("Hello, world!"),
        unknown_command => eprintln!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown_command
        ),
    }
}
