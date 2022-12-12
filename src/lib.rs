use std::fs;
use std::error::Error;

pub struct Config {
    pub recherche: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let recherche = args[1].clone();
        let file = args[2].clone();

        Ok(Config { recherche, file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contenu = fs::read_to_string(config.file)?;

    println!("In the text :\n{}", contenu);
    Ok(())
}