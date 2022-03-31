pub mod cargoreader;
mod commands;
pub mod common;
pub mod crateinfo;
pub mod cratesio;
pub mod utils;

use crate::common::version_req_str;
use crate::utils::Join;
use std::env;

fn get_dependencies() -> utils::Result<Vec<crateinfo::CrateInfo>> {
    cargoreader::read_cargo_file().and_then(|cargo_file| cargoreader::parse_cargo_file(cargo_file))
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: carp <command>");
        return;
    }

    match commands::parse_args(&args[1..]) {
        Ok(command) => match command.name.as_str() {
            "list" => match get_dependencies() {
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
            "check" => match get_dependencies() {
                Ok(dependencies) => {
                    let check_all_deps = command.args.len() == 0;
                    let deps_to_check: Vec<_> = dependencies
                        .iter()
                        .filter(|crate_info| {
                            check_all_deps || command.args.iter().any(|arg| arg == &crate_info.name)
                        })
                        .collect();
                    if !check_all_deps && deps_to_check.len() != command.args.len() {
                        let unknown_deps = command
                            .args
                            .into_iter()
                            .filter(|arg| {
                                !deps_to_check
                                    .iter()
                                    .any(|crate_info| arg == &crate_info.name)
                            })
                            .join(",");
                        eprintln!(
                            "ERROR checking out of date dependencies: Crate(s) '{}' not found.",
                            unknown_deps
                        );
                        return;
                    }
                    match cratesio::get_index().and_then(|index| {
                        cratesio::out_of_date_crate_infos(
                            command.flags.strict,
                            command.flags.only_strict,
                            &index,
                            &deps_to_check,
                        )
                    }) {
                        Ok(out_of_date) => {
                            if out_of_date.len() == 0 {
                                println!("Everything is up to date!")
                            } else {
                                for dependency in out_of_date {
                                    println!(
                                        "! {} ({})",
                                        dependency.name,
                                        version_req_str(&dependency.version_req)
                                    )
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("ERROR checking out of date dependencies: {}", err)
                        }
                    }
                }
                Err(err) => eprintln!("ERROR reading dependencies: {}", err),
            },
            unknown_command => eprintln!("Unknown command: {}", unknown_command),
        },
        Err(err) => eprintln!("ERROR parsing command: {}", err),
    }
}
