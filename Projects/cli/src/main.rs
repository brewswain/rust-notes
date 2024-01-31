use std::{env, process};

use cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turns the iterator into a vector containing all the values produced by the iterator.

    let config = Config::build(&args).unwrap_or_else(|err| {
        // Using unwrap_or_else allows us to define some custom, non-panic! error handling. If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to unwrap_or_else.
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = cli::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
