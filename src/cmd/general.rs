use std::{env, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

pub fn get_directory(args: &ArgMatches) -> PathBuf {
    if let Some(p) = args.get_one::<PathBuf>("dir") {
        if p.is_relative() {
            env::current_dir().unwrap().join(p)
        } else {
            p.to_path_buf()
        }
    } else {
        // if dir is not in args -> default current!
        env::current_dir().expect("unable get current dir!")
    }
}
