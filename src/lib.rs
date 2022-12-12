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

    for ligne in rechercher(&config.recherche, &contenu) {
        println!("{}", ligne);
    }
    
    Ok(())
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let mut resultat = Vec::new();


    for ligne in contenu.lines() {
        if ligne.contains(recherche) {
            resultat.push(ligne);
        }
    } 
    resultat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_resultat() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.";

        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            rechercher(recherche, contenu)
        );
    }
}