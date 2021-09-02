use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.filename)?;

    // println!("File content: \n{}", content);
	for line in search(config.query, &content) {
		println!("{}", line);
	}
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

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&'a str> = Vec::<&'a str>::new();
	for line in content.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}