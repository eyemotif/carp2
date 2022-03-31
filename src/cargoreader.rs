use crate::common::get_cargo_path;
use crate::crateinfo::{get_versions_from_str, CrateInfo, RawToml};
use crate::utils::Result;
use std::fs;
use toml::Value;

fn parse_dependency_value<'a>(name: &str, value: Value) -> Result<CrateInfo> {
    if let Some(string) = value.as_str() {
        let (version_req, version) = get_versions_from_str(string)?;
        Ok(CrateInfo {
            name: name.to_string(),
            version,
            version_req,
            raw_toml_value: RawToml::String(value),
        })
    } else if let Some(table) = value.as_table() {
        if let Some(ver) = table
            .get("version")
            .ok_or(format!(
                "Could not locate the 'version' key in the dependency '{}'.",
                name
            ))?
            .as_str()
        {
            let (version_req, version) = get_versions_from_str(ver)?;
            Ok(CrateInfo {
                name: name.to_string(),
                version,
                version_req,
                raw_toml_value: RawToml::Table(value),
            })
        } else {
            Err(format!(
                "The 'version' key in the dependency '{}' is not a string.",
                name
            )
            .into())
        }
    } else {
        Err(format!(
            "The value for the key '{}' is neither a string nor a table.",
            name
        )
        .into())
    }
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
        .ok_or("Could not parse the dependencies value to a table in the Cargo.toml file given.")?
        .to_owned();
    let mut dependencies: Vec<CrateInfo> = vec![];

    for (k, v) in dependencies_table {
        dependencies.push(parse_dependency_value(&k, v)?);
    }
    Ok(dependencies)
}

pub fn write_dependencies(dependencies: Vec<CrateInfo>) -> Result<()> {
    let mut cargo_file = read_cargo_file()?;
    let mut cargo_deps = cargo_file
        .get("dependencies")
        .ok_or("Could not locate the dependencies value in the Cargo.toml file given.")?
        .to_owned();
    let cargo_deps_table = cargo_deps
        .as_table_mut()
        .ok_or("Could not parse the dependencies value to a table in the Cargo.toml file given.")?;

    for crate_info in dependencies {
        cargo_deps_table.insert(
            crate_info.name,
            match crate_info.raw_toml_value {
                RawToml::String(string) => string,
                RawToml::Table(table) => table,
            },
        );
    }

    cargo_file["dependencies"] = Value::from(cargo_deps_table.to_owned());
    let new_cargo_file = toml::ser::to_string(&cargo_file)?;
    Ok(fs::write(get_cargo_path(), new_cargo_file)?)
}
