use crate::utils::Result;
use semver::{Op, Version, VersionReq};
use toml::Value;

#[derive(Debug)]
pub enum RawToml<'a> {
    String(&'a Value),
    Table(&'a Value),
}

#[derive(Debug)]
pub struct CrateInfo<'a> {
    pub name: String,
    pub version_req: VersionReq,
    pub version: Option<Version>,
    pub raw_toml_value: RawToml<'a>,
}

pub fn get_versions_from_str(ver_str: &str) -> Result<(VersionReq, Option<Version>)> {
    let version_req: VersionReq = ver_str.parse()?;
    let version = {
        let mut result = None;
        for comparator in &version_req.comparators {
            match comparator.op {
                Op::Exact | Op::Tilde | Op::Caret => {
                    result = Some(Version::new(
                        comparator.major,
                        comparator.minor.unwrap(),
                        comparator.minor.unwrap(),
                    ));
                    break;
                }
                _ => (),
            }
        }
        result
    };
    Ok((version_req, version))
}
