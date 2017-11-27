extern crate minigrep;

use std::env;
use std::process;
use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}