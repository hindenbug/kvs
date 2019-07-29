extern crate clap;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process::exit;

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

    match matches.subcommand_name() {
        Some("set") => {
            panic!("unimplemented");
            exit(1)
        }
        Some("get") => {
            panic!("unimplemented");
            exit(1)
        }
        Some("rm") => {
            panic!("unimplemented");
            exit(1)
        }
        _ => {
            eprintln!("command Not Found");
            exit(1)
        }
    }
}
