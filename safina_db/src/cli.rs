use clap::{arg, Command};

pub fn cli_parser() -> Command {
    Command::new("safina_db")
        .version("1.0.0")
        .about("'Safina DB' a persistent Key-Value store")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("insert")
                .about("Inserts a new entry")
                .arg_required_else_help(true)
                .arg(arg!(key: [KEY]).required(true))
                .arg(arg!(value: [VALUE]).required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("get entry value by key")
                .arg_required_else_help(true)
                .arg(arg!(key: [KEY]).required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("delete entry value by key")
                .arg_required_else_help(true)
                .arg(arg!(key: [KEY]).required(true)),
        )
        .subcommand(
            Command::new("update")
                .about("update entry value")
                .arg_required_else_help(true)
                .arg(arg!(base: [KEY]).required(true))
                .arg(arg!(head: [VALUE]).required(true)),
        )
}
