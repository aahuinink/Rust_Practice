use std::env;
use std::process;

use minigrep::GrepObject;

fn main() {
    let grep_object: GrepObject = GrepObject::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_object.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

