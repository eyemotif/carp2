pub mod cargoreader;
mod commands;
pub mod common;
pub mod crateinfo;
pub mod cratesio;
pub mod utils;

use common::version_req_str;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: carp <command>");
        return;
    }

    match commands::parse_args(&args[1..]) {
        Ok(command) => match command.name.as_str() {
            "list" => match cargoreader::read_cargo_file()
                .and_then(|cargo_file| cargoreader::parse_cargo_file(cargo_file))
            {
                Ok(dependencies) => {
                    for dependency in dependencies {
                        println!(
                            "{} ({})",
                            dependency.name,
                            version_req_str(&dependency.version_req)
                        )
                    }
                }
                Err(err) => eprintln!("ERROR reading dependencies: {}", err),
            },
            unknown_command => eprintln!("Unknown command: {}", unknown_command),
        },
        Err(err) => eprintln!("ERROR parsing command: {}", err),
    }
}
