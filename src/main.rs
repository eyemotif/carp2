pub mod cargoreader;
pub mod common;
pub mod crateinfo;
pub mod cratesio;
pub mod utils;

use std::env;
use utils::Result;

fn check_cargo<'a>(
    file: &'a Vec<crateinfo::CrateInfo>,
) -> Result<Vec<&'a crateinfo::CrateInfo<'a>>> {
    let index = crates_index::Index::new_cargo_default()?;
    let out_of_date: Vec<_> = cratesio::out_of_date_crate_infos(false, &index, &file)?;
    Ok(out_of_date)
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: carp <command>");
        return;
    }
    match args[1].to_lowercase().as_str() {
        "test" => match cargoreader::read_cargo_file() {
            Ok(file) => match cargoreader::parse_cargo_file(&file) {
                Ok(cargo) => match check_cargo(&cargo) {
                    Ok(parsed) => println!("Ok: {:?}", parsed),
                    Err(err) => eprintln!("Check error: {}", err),
                },
                Err(err) => eprintln!("Get error: {}", err),
            },
            Err(err) => eprintln!("Read error: {}", err),
        },
        unknown_command => eprintln!(
            "Unknown command '{}'. Use 'carp help' for a list of commands.",
            unknown_command
        ),
    }
}
