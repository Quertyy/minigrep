use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error during the args' interpretation : {}", err);
        process::exit(1);
    });


    if let Err(e) = minigrep::run(config) {
        println!("Erreur applicative: {}", e);

        process::exit(1);
    }
}

