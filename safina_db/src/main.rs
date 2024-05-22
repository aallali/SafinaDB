use std::ffi::OsString;
use std::io::{self, Write};
mod cli;
use cli::cli_parser;
mod kv_store;
use kv_store::Store;
use regex::Regex;

fn main() {
    let mut store = Store::new();

    loop {
        print!("\n(safina) ➜ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Define the regex pattern to match words or quoted phrases
        let re = Regex::new(r#""([^"]*)"|(\S+)"#).unwrap();

        // Use the regex to find all matches
        let mut args: Vec<String> = re
            .captures_iter(input)
            .map(|cap| {
                cap.get(1).map_or_else(
                    || cap.get(2).unwrap().as_str().to_string(),
                    |m| m.as_str().to_string(),
                )
            })
            .collect();
        args.insert(0, "safina_db".to_string());

        let matches = match cli_parser().try_get_matches_from(args) {
            Ok(matches) => matches,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match matches.subcommand() {
            Some(("insert", sub_matches)) => {
                let key: &str = sub_matches
                    .get_one::<String>("key")
                    .map(|s| s.as_str())
                    .unwrap();

                let value: &str = sub_matches
                    .get_one::<String>("value")
                    .map(|s| s.as_str())
                    .unwrap();

                // println!("{:#?}", store);
                match store.insert(String::from(key), String::from(value)) {
                    Ok(_) => println!("Inserted entry {{'{key}' : '{value}'}}"),
                    Err(e) => println!("Error {}", e),
                }
            }

            Some(("get", sub_matches)) => {
                let key = sub_matches
                    .get_one::<String>("key")
                    .map(|s| s.as_str())
                    .unwrap();
                // println!("{:#?}", store);

                match store.get(key) {
                    Ok(pair) => println!("Entry: {key} = {}", pair.value),
                    Err(e) => println!("Error: {}", e),
                }
            }

            Some(("update", sub_matches)) => {
                let key: &str = sub_matches
                    .get_one::<String>("key")
                    .map(|s| s.as_str())
                    .unwrap();

                let value: &str = sub_matches
                    .get_one::<String>("value")
                    .map(|s| s.as_str())
                    .unwrap();

                // println!("{:#?}", store);
                match store.update(key, value) {
                    Ok(()) => println!("Entry updated succeffuly"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            Some(("delete", sub_matches)) => {
                let key: &str = sub_matches
                    .get_one::<String>("key")
                    .map(|s| s.as_str())
                    .unwrap();

                // println!("{:#?}", store);
                store.delete(key);
                println!("Entry deleted succeffuly")
            }
            Some(("exit", _sub_matches)) => {
                println!("exit...");
                break;
            }
            Some((ext, sub_matches)) => {
                let args = sub_matches
                    .get_many::<OsString>("")
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                println!("Calling out to {ext:?} with {args:?}");
            }
            _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
        }
    }
}
