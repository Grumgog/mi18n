use clap::{crate_authors, crate_name, crate_version, Command};

mod cmd;

fn main() {
    let command = create_clap_command();

    let res = match command.get_matches().subcommand() {
        Some(("test", sub_matches)) => cmd::test::execute(sub_matches),
        Some(("edit", sub_matches)) => cmd::edit::execute(sub_matches),
        _ => i32::from(5),
    };
}

pub fn create_clap_command() -> Command {
    let app = Command::new(crate_name!())
        .about("json i18n manager")
        .author(crate_authors!())
        .version(crate_version!())
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(cmd::test::make_subcommand())
        .subcommand(cmd::edit::make_subcommand());

    app
}
