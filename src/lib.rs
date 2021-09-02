use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.filename)?;

    println!("File content: \n{}", content);
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String
}

impl Config<'_> {
    pub fn new<'a>(args: &'a[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: &'a String = &args[1];
        let filename: &'a String = &args[2];
        Ok(Config {query: &query, filename: &filename})
    }
}