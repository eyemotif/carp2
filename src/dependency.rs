use crate::utils::Result;
use semver::{Op, Version, VersionReq};
use toml::Value;

#[derive(Debug)]
pub enum RawToml {
    String(Value),
    Table(Value),
}

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version_req: VersionReq,
    pub version: Option<Version>,
    pub raw_toml_value: RawToml,
}

pub fn get_version_from_version_req(version_req: &VersionReq) -> Option<Version> {
    let mut result = None;
    for comparator in &version_req.comparators {
        match comparator.op {
            Op::Exact | Op::Tilde | Op::Caret => {
                result = Some(Version::new(
                    comparator.major,
                    comparator.minor.unwrap(),
                    comparator.patch.unwrap(),
                ));
                break;
            }
            _ => (),
        }
    }
    result
}

pub fn get_versions_from_str(ver_str: &str) -> Result<(VersionReq, Option<Version>)> {
    let version_req: VersionReq = ver_str.parse()?;
    let version = get_version_from_version_req(&version_req);
    Ok((version_req, version))
}

pub fn transform_dependency_version(ver_str: &str, dependency: Dependency) -> Result<Dependency> {
    let (version_req, version) = get_versions_from_str(ver_str)?;
    let transformed_raw_toml = match dependency.raw_toml_value {
        RawToml::String(_) => RawToml::String(Value::from(ver_str)),
        RawToml::Table(mut table) => {
            let new_table = table
                .as_table_mut()
                .expect("Raw Toml Value marked as Table is not Table.");
            new_table.insert("version".to_owned(), Value::from(ver_str));
            RawToml::Table(Value::from(new_table.to_owned()))
        }
    };
    Ok(Dependency {
        name: dependency.name,
        version_req,
        version,
        raw_toml_value: transformed_raw_toml,
    })
}
