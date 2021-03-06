pub mod cargoreader;
mod command;
pub mod common;
pub mod cratesio;
pub mod dependency;
pub mod utils;

use crate::common::version_req_str;
use crate::utils::{Join, One, Result};
use dependency::{transform_dependency_version, Dependency};
use std::env;

fn get_dependencies() -> Result<Vec<Dependency>> {
    cargoreader::read_cargo_file().and_then(|cargo_file| cargoreader::parse_cargo_file(cargo_file))
}

fn filter_dependencies(
    check: &Vec<String>,
    dependencies: Vec<Dependency>,
) -> Result<Vec<Dependency>> {
    let check_all_deps = check.len() == 0;
    let deps_to_check: Vec<_> = dependencies
        .into_iter()
        .filter(|dependency| check_all_deps || check.iter().any(|arg| arg == &dependency.name))
        .collect();
    if check_all_deps || deps_to_check.len() == check.len() {
        Ok(deps_to_check)
    } else {
        let unknown_deps = check
            .into_iter()
            .filter(|arg| {
                !deps_to_check
                    .iter()
                    .any(|dependency| *arg == &dependency.name)
            })
            .join(",");
        Err(format!("Crate(s) '{}' not found.", unknown_deps).into())
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: carp <command>");
        return;
    }

    match command::parse_args(&args[1..]) {
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
                    match filter_dependencies(&command.args, dependencies).and_then(
                        |deps_to_check| {
                            cratesio::get_index().and_then(|index| {
                                cratesio::out_of_date_dependencies(
                                    command.flags.strict,
                                    command.flags.only_strict,
                                    &index,
                                    &deps_to_check,
                                )
                            })
                        },
                    ) {
                        Ok(out_of_date) => {
                            if out_of_date.len() == 0 {
                                println!("Everything is up to date!")
                            } else {
                                for (dependency, latest_version) in out_of_date {
                                    println!(
                                        "! {} ({}): ({})",
                                        dependency.name,
                                        version_req_str(&dependency.version_req),
                                        latest_version
                                    )
                                }
                            }
                        }
                        Err(err) => eprintln!("ERROR checking out of date dependencies: {}", err),
                    }
                }
                Err(err) => eprintln!("ERROR reading dependencies: {}", err),
            },
            "add" => {
                if command.args.len() < 1 {
                    eprintln!("Usage: carp add <crate> [version]");
                    return;
                }
                match cratesio::get_index().and_then(|index| {
                    if let Some(crte) = index.crate_(&command.args[0]) {
                        cratesio::get_crate_latest_versions(&crte)
                    } else {
                        Err(format!("Could not find crate '{}'", &command.args[0]).into())
                    }
                }) {
                    Ok((version_req, version)) => {
                        let new_dependency = if command.args.len() > 1 {
                            let toml_value_str = &command.raw_args[command.args[0].len()..];

                            // TODO: why doesnt this parse anything?
                            match toml_value_str.parse::<toml::Value>() {
                                Ok(toml_value) => {
                                    match cargoreader::parse_dependency_value(
                                        &command.args[0],
                                        toml_value,
                                    ) {
                                        Ok(dependency) => dependency,
                                        Err(err) => {
                                            eprintln!(
                                                "ERROR parsing arguments to a dependency: {}",
                                                err
                                            );
                                            return;
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("ERROR parsing arguments into a TOML value: {}", err);
                                    return;
                                }
                            }
                        } else {
                            Dependency {
                                name: command.args[0].to_owned(),
                                version_req: version_req.clone(),
                                version: version.clone(),
                                raw_toml_value: dependency::RawToml::String(
                                    format!("{}", version_req_str(&version_req)).into(),
                                ),
                            }
                        };
                        match cargoreader::read_cargo_file()
                            .and_then(|cargo_file| cargoreader::parse_cargo_file(cargo_file))
                        {
                            Ok(dependencies) => {
                                let mut new_dependencies = dependencies;
                                new_dependencies.push(new_dependency);
                                match cargoreader::write_dependencies(new_dependencies) {
                                    Ok(()) => (),
                                    Err(err) => eprintln!("ERROR writing dependencies: {}", err),
                                }
                            }
                            Err(err) => eprintln!("ERROR reading dependencies: {}", err),
                        }
                    }
                    Err(err) => eprintln!("ERROR finding crate: '{}'", err),
                }
            }
            "rem" => {
                if command.args.len() != 1 {
                    eprintln!("Usage: carp rem <dependency>");
                    return;
                }
                match get_dependencies() {
                    Ok(dependencies) => {
                        if dependencies
                            .iter()
                            .any(|dependency| dependency.name == command.args[0])
                        {
                            let new_dependencies = dependencies
                                .into_iter()
                                .filter(|dependency| dependency.name != command.args[0]);
                            match cargoreader::write_dependencies(new_dependencies.collect()) {
                                Ok(()) => println!("- {}", command.args[0]),
                                Err(err) => eprintln!("ERROR writing dependencies: {}", err),
                            }
                        } else {
                            eprintln!(
                                "ERROR removing dependency: Dependency '{}' not found",
                                command.args[0]
                            );
                        }
                    }
                    Err(err) => eprintln!("ERROR reading dependencies: {}", err),
                }
            }
            "update" => match get_dependencies() {
                Ok(dependencies) => {
                    match filter_dependencies(&command.args, dependencies.clone()).and_then(
                        |deps_to_check| {
                            cratesio::get_index().and_then(|index| {
                                cratesio::out_of_date_dependencies(
                                    command.flags.strict,
                                    command.flags.only_strict,
                                    &index,
                                    &deps_to_check,
                                )
                            })
                        },
                    ) {
                        Ok(out_of_date) => {
                            if out_of_date.len() == 0 {
                                println!("Everything is up to date!")
                            } else {
                                for (dependency, latest_version) in &out_of_date {
                                    println!(
                                        "* {} ({}) -> ({})",
                                        dependency.name,
                                        version_req_str(&dependency.version_req),
                                        latest_version
                                    )
                                }
                                let update_result: Result<Vec<Dependency>> = dependencies
                                    .into_iter()
                                    .map(|dependency| {
                                        if let Some((_, new_ver)) = out_of_date
                                            .iter()
                                            .one(|(ood, _)| ood.name == dependency.name)
                                        {
                                            transform_dependency_version(
                                                &new_ver.to_string(),
                                                dependency,
                                            )
                                        } else {
                                            Ok(dependency)
                                        }
                                    })
                                    .collect();

                                match update_result {
                                    Ok(updated_deps) => {
                                        match cargoreader::write_dependencies(updated_deps) {
                                            Ok(()) => (),
                                            Err(err) => {
                                                eprintln!("ERROR writing dependencies: {}", err)
                                            }
                                        }
                                    }
                                    Err(err) => eprintln!("ERROR updating dependencies: {}", err),
                                }
                            }
                        }
                        Err(err) => eprintln!("ERROR checking out of date dependencies: {}", err),
                    }
                }
                Err(err) => eprintln!("ERROR reading dependencies: {}", err),
            },

            unknown_command => eprintln!("Unknown command: {}", unknown_command),
        },
        Err(err) => eprintln!("ERROR parsing command: {}", err),
    }
}
