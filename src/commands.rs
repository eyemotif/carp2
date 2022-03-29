use crate::utils::Result;

pub struct CommandFlags {
    pub strict: bool,
}
pub struct Command {
    pub name: String,
    pub flags: CommandFlags,
    pub args: Vec<String>,
    pub raw_args: String,
}

pub fn parse_args(args: &[String]) -> Result<Command> {
    assert!(args.len() > 0, "Expected at least one argument.");

    let mut filtered_args = vec![];
    let mut flags = CommandFlags { strict: false };

    for arg in args {
        if arg.starts_with("-") {
            match &arg[1..] {
                "-strict" | "s" => flags.strict = true,
                unknown_flag => return Err(format!("Unknown flag '{}'.", unknown_flag).into()),
            }
        } else {
            filtered_args.push(arg.to_owned())
        }
    }

    let raw_args = filtered_args
        .iter()
        .fold(String::new(), |acc, i| format!("{}{} ", acc, i))
        .trim()
        .to_owned();

    Ok(Command {
        name: args[0].to_owned(),
        flags,
        args: filtered_args,
        raw_args,
    })
}
