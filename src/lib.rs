pub mod validators;
pub mod engine;

use std::fs;
use std::error::Error;
use std::env;
pub use validators::{is_valid_flags, print_flags};


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.filename)?;

    let results = if config.case_insentitive {
		engine::search_case_insensitive(&config.query, &content)
	} else {
		engine::search(&config.query, &content)
	};

	for line in results {
		println!("{}", line);
	}
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
	pub case_insentitive: bool,
}

impl Config {

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		
		args.next();

        let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string")
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a filename string")
		};

		let case_insentitive = Self::is_case_insensitive(args);
        Ok(Config { query, filename, case_insentitive })
    }

	fn is_case_insensitive(mut args: env::Args) -> bool {

		if match args.next() {
			Some(arg) => arg == "-i" || arg == "insensitive",
			_ => false
		} {
			return true;
		}

		if let Ok(v) = env::var("CASE_INSENSITIVE") {
			return v == "1";
		}

		false
	}

}
