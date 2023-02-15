use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::cmd::general::get_directory;

pub fn make_subcommand() -> Command {
    let command = Command::new("test").about("Testing your i18n files").arg(
        Arg::new("dir")
            .short('d')
            .value_name("dir")
            .value_parser(clap::value_parser!(PathBuf))
            .help("working directory"),
    );

    command
}

pub fn execute(args: &ArgMatches) -> i32 {
    println!("{:#?}", args);
    let path = get_directory(args);
    println!("{:#?}", path);

    5
}
