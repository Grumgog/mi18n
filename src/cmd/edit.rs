use clap::{ArgMatches, Command};

pub fn make_subcommand() -> Command {
    let command = Command::new("edit");

    command
}

pub fn execute(args: &ArgMatches) -> i32 {
    5
}
