use std::env;
use std::path::PathBuf;

pub fn get_cargo_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("cargo.toml");
    path
}
