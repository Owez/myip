use curl::{self, easy::Easy};
use std::io::{stdout, Write};
use std::{env, process};

const HELP: &str = "Usage: myip [OPTIONS]\n\n  Simple ip address fetching, designed for scripts\n\nOptions:\n    --help, -h      Shows this message\n    --verbose, -v   Enables verbose mode for errors";
const API_URL: &str = "127.0.0.1:5000/get";

fn get_ip() -> Result<(), curl::Error> {
    let mut easy = Easy::new();

    easy.url(API_URL)?;
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;

    easy.perform()
}

fn main() {
    let mut verbose = false;
    let mut args = env::args();

    args.next();

    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            "help" | "-h" | "--help" => {
                println!("{}", HELP);
                process::exit(0)
            }
            unknown => {
                eprintln!("{}\nThe arg '{}' is unrecognized!", HELP, unknown);
                process::exit(1)
            }
        }
    }

    match get_ip() {
        Ok(()) => (),
        Err(err) => {
            if verbose {
                eprintln!(
                    "Error found whilst getting using myip:\n  {}",
                    match err.extra_description() {
                        Some(extra) => extra,
                        None => err.description(),
                    }
                );

                if err.description().contains("resolve host") {
                    eprintln!("  Perhaps the myip server is unreachable?")
                }
            };
            process::exit(1)
        }
    }
}
