use crate::kv_store::Store;
use crate::STORE;
use clap::{arg, Command};
use std::io::Write;
use std::sync::MutexGuard;

/// Runs the REPL loop, reading user input and responding accordingly.
///
/// # Returns
/// * `Ok(())` if the REPL exits successfully.
/// * `Err(String)` if an error occurs during execution.
pub fn run() -> Result<(), String> {
    loop {
        let line: String = readline()?;
        let line: &str = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

/// Processes the user input and executes the corresponding command.
///
/// # Arguments
/// * `line` - The input line entered by the user.
///
/// # Returns
/// * `Ok(bool)` - A boolean indicating whether to quit the REPL.
/// * `Err(String)` - An error message if the input processing fails.
fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;

    let mut store: MutexGuard<Store> = STORE.lock().map_err(|e| e.to_string())?;

    match matches.subcommand() {
        Some(("insert", sub_matches)) => {
            // Handle the 'insert' command to add a new key-value pair to the store
            let key: &str = sub_matches
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap();

            let value: &str = sub_matches
                .get_one::<String>("value")
                .map(|s| s.as_str())
                .unwrap();

            match store.insert(String::from(key), String::from(value)) {
                Ok(_) => println!("Inserted entry {{'{key}': '{value}'}}"),
                Err(e) => println!("Error {}", e),
            }
        }

        Some(("get", sub_matches)) => {
            // Handle the 'get' command to retrieve a value by its key from the store
            let key = sub_matches
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap();

            match store.get(key) {
                Ok(pair) => println!("Entry: {{\"{key}\" : \"{}\"}}", pair.value),
                Err(e) => println!("Error: {}", e),
            }
        }

        Some(("update", sub_matches)) => {
            // Handle the 'update' command to modify the value associated with a key
            let key: &str = sub_matches
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap();

            let value: &str = sub_matches
                .get_one::<String>("value")
                .map(|s| s.as_str())
                .unwrap();

            match store.update(key, value) {
                Ok(_) => println!("Updated entry {{'{key}' : '{value}'}}"),
                Err(e) => println!("Error {}", e),
            }
        }

        Some(("delete", sub_matches)) => {
            // Handle the 'delete' command to remove a key-value pair from the store
            let key: &str = sub_matches
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap();

            store.delete(key);
            println!("Entry deleted successfully")
        }
        Some(("quit", _matches)) => {
            // Handle the 'quit' command to exit the REPL
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Some((name, _matches)) => {
            // Handle any unimplemented commands
            println!("Method [{name}] Not implmeneted")
        }
        // This case should never occur because `subcommand_required(true)` ensures a subcommand is always provided.
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

/// Defines the command-line interface using Clap.
///
/// # Returns
/// A `Command` instance representing the CLI structure.
fn cli() -> Command {
    Command::new("safina_db")
        .version("1.0.0")
        .multicall(true)
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
                .arg(arg!(key: [KEY]).required(true))
                .arg(arg!(value: [VALUE]).required(true)),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .alias("by")
                .about("Quit the REPL"),
        )
}

/// Reads a line of input from the user.
///
/// # Returns
/// * `Ok(String)` - The input line entered by the user.
/// * `Err(String)` - An error message if reading input fails.
fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "\n(safinaDB) ➜ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
