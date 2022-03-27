use crate::common::get_cargo_path;
use crate::utils::Result;
use semver::{Version, VersionReq};
use std::fs;
use toml::Value;

#[derive(Debug)]
pub struct CrateInfo<'a> {
    pub name: String,
    pub version: Version,
    pub version_req: VersionReq,
    pub raw_toml_value: &'a Value,
}

fn parse_dependency_value<'a>(name: &str, value: &'a Value) -> Result<CrateInfo<'a>> {
    if let Some(string) = value.as_str() {
        Ok(CrateInfo {
            name: name.to_string(),
            version: string.parse()?,
            version_req: string.parse()?,
            raw_toml_value: value,
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
            Ok(CrateInfo {
                name: name.to_string(),
                version: ver.parse()?,
                version_req: ver.parse()?,
                raw_toml_value: value,
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

pub fn parse_cargo_file(file_value: &Value) -> Result<Vec<CrateInfo>> {
    let dependencies_table = file_value
        .get("dependencies")
        .ok_or("Could not locate the dependencies value in the Cargo.toml file given.")?
        .as_table()
        .ok_or("Could not parse the dependencies value to a table in the Cargo.toml file given.")?;
    let mut dependencies: Vec<CrateInfo> = vec![];

    for (k, v) in dependencies_table {
        dependencies.push(parse_dependency_value(k, v)?);
    }
    Ok(dependencies)
}
