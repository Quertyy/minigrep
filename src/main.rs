use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error during the args' interpretation : {}", err);
        process::exit(1);
    });

    println!("Le mot que je recherche est: {}", config.recherche);
    println!("Le fichier dans lequel on cherche est: {}", config.file);

    if let Err(e) = minigrep::run(config) {
        println!("Erreur applicative: {}", e);

        process::exit(1);
    }
}

