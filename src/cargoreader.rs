use crate::common::get_cargo_path;
use crate::utils::Result;
use semver::VersionReq;
use std::fs;
use toml::Value;

#[derive(Debug)]
pub struct CrateInfo {
    name: String,
    semver: VersionReq,
}

pub fn read_cargo_file() -> Result<Value> {
    let cargo_file = fs::read_to_string(get_cargo_path())?;
    let parse: Value = cargo_file.parse()?;
    Ok(parse)
}

pub fn parse_cargo_file(file_value: Value) -> Result<Vec<CrateInfo>> {
    let dependencies_table = file_value
        .get("dependencies")
        .ok_or("Could not locate the dependencies value in the Cargo.toml file given.")?
        .as_table()
        .ok_or("Could not parse the dependencies value to a table in the Cargo.toml file given.")?;
    let mut dependencies: Vec<CrateInfo> = vec![];

    for (k, v) in dependencies_table {
        let semver_parse: VersionReq = v
            .as_str()
            .ok_or(format!("The value for key '{}' is not a string.", k))?
            .parse()?;
        dependencies.push(CrateInfo {
            name: k.to_string(),
            semver: semver_parse,
        })
    }
    Ok(dependencies)
}
