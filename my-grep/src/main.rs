use std::{env, process};

use my_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = my_grep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

