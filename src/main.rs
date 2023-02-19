use clap::{crate_authors, crate_name, crate_version, Command};
mod general;
mod cmd;
mod json_test;

fn main() {
    let command = create_clap_command();

    match command.get_matches().subcommand() {
        Some(("test", sub_matches)) => cmd::test::execute(sub_matches),
        _ => unreachable!(),
    };
}

pub fn create_clap_command() -> Command {
    let command = Command::new(crate_name!())
        .about("json i18n manager")
        .author(crate_authors!())
        .version(crate_version!())
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(cmd::test::make_subcommand());

    command
}
