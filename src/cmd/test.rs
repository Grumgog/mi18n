use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};
use std::ffi::OsStr;

use std::fs::read_dir;

use crate::cmd::general::get_directory;
use crate::json_test;

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
    let path = get_directory(args);

    let dir_content = read_dir(&path).expect("cannot read dir!");

    let mut files: Vec<PathBuf> = vec![];

    for file in dir_content {
        let file_info = file.expect("cannot get info for file");
        let filepath = file_info.path();
        let extension = if let Some(path) = filepath.extension() {
            path
        } else {
            OsStr::new("")
        };

        if extension == "json" {
            println!("finnaly json! as {:#?}", file_info.path());
            files.push(file_info.path());
        }
    }

    if files.len() > 0 {
        json_test::test::test_files(&files);
    } else {
        println!("No json files find!");
    }

    5
}
