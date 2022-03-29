use semver::VersionReq;
use std::env;
use std::path::PathBuf;

pub fn get_cargo_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("cargo.toml");
    path
}

pub fn version_req_str(version_req: &VersionReq) -> String {
    let string = format!("{}", version_req);
    if string.contains("^") {
        string.replace("^", "")
    } else {
        string
    }
}
