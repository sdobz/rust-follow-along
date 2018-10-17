extern crate ch13b_minigrep2;

use std::env;
use std::process;

use ch13b_minigrep2::Config;

fn main() {
    // CH13 UPDATED
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ch13b_minigrep2::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
