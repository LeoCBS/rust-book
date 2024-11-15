use core::panic;
use std::{env, error::Error, fs, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        //print error to standard Error instead to standard output
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //using if let because run doesn't return any value, just a error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
