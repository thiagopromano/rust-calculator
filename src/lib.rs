use std::error::Error;
use crate::lexer::*;
use crate::sintatical::*;

mod lexer;
mod sintatical;
mod token;

pub struct Config {
    pub input: String,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let input = args[1].clone();

        Ok(Config { input })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (errors, tokens) = lexic_analize(&config.input[..]);
    for e in &errors {
        println!("{}", e)
    }
    for t in &tokens {
        println!("Received a {:#?} token", t)
    }
    let errors = syntatic_analize(&tokens);
    for e in &errors {
        println!("{}", e)
    }
    Ok(())
}
