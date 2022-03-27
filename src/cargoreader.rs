use crate::common::get_cargo_path;
use crate::utils::Result;
use semver::VersionReq;
use std::fs;

pub struct CrateInfo {
    name: String,
    semver: VersionReq,
}

pub fn read_cargo_file() -> Result<String> {
    let cargo_file = fs::read_to_string(get_cargo_path())?;
    Ok(cargo_file)
}
