extern crate clap;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(
                    Arg::with_name("key")
                        .help("The key name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("value")
                        .help("The value")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get a value for a key.")
                .arg(
                    Arg::with_name("key")
                        .help("Key name")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm").about("Remove a given key").arg(
                Arg::with_name("key")
                    .help("Key name")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    let mut store = KvStore::new();
    let key = matches.value_of("key").unwrap().to_string();

    match matches.subcommand_name() {
        Some("set") => {
            let value = matches.value_of("value").unwrap().to_string();
            store.set(key, value);
        }
        Some("get") => match store.get(key) {
            Some(val) => println!("#{}", val),
            None => println!("Not Found"),
        },
        Some("rm") => {
            store.remove(key);
        }
        _ => {
            eprintln!("command Not Found");
            process::exit(1)
        }
    }
}
