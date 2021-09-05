pub mod validators;
pub mod engine;

use std::fs;
use std::error::Error;
use std::env;
pub use validators::{is_valid_flags, print_flags};


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.filename)?;

    let results = if config.case_insentitive {
		engine::search_case_insensitive(config.query, &content)
	} else {
		engine::search(config.query, &content)
	};

	for line in results {
		println!("{}", line);
	}
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
	pub case_insentitive: bool,
}

impl Config<'_> {

    pub fn new<'a>(args: &'a[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: &'a String = &args[1];
        let filename: &'a String = &args[2];
		let case_insentitive = Self::is_case_insensitive(&args);
        Ok(Config {query: &query, filename: &filename, case_insentitive: case_insentitive })
    }

	fn is_case_insensitive(args: &[String]) -> bool {

		if let Some(_) = args.iter().position(|arg| arg == "-i" || arg == "--insensitive") {
			return true;
		}

		if let Ok(v) = env::var("CASE_INSENSITIVE") {
			return v == "1";
		}

		false
	}

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn is_case_insensitive() {
		assert!(Config::is_case_insensitive(&["-i".to_string()]));
		assert!(Config::is_case_insensitive(&["--insensitive".to_string()]));
	}

}