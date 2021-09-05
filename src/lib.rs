use std::fs;
use std::error::Error;
use std::env;


const VALID_FLAGS: [&str; 2] = ["-i", "--insensitive"];

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.filename)?;

    let results = if config.case_insentitive {
		search_case_insensitive(config.query, &content)
	} else {
		search(config.query, &content)
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

pub fn is_valid_flags(args: &[String]) -> Result<(), String> {
		
	let flags: Vec<&String> = args.iter().filter(|arg| arg.starts_with("-")).collect();
	let mut invalid_flags: Vec<String> = Vec::<String>::new();
	for flag in flags {
		if !VALID_FLAGS.contains(&&flag[..]) {
			invalid_flags.push(flag.to_string());
		}
	}
	if invalid_flags.len() > 0 {
		return Err(format!("Invalid flags: {}", invalid_flags.join(",")));
	}
	Ok(())
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

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&'a str> = Vec::<&'a str>::new();
	let query: String = query.to_lowercase();
	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
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

	#[test]
	fn case_insentitive(){
		let query = "rUst";
		let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		
		assert_eq!(
			vec!["Rust:", "Trust me."],
				search_case_insensitive(query, contents)
			);
	}

	#[test]
	fn is_case_insensitive() {
		assert!(Config::is_case_insensitive(&["-i".to_string()]));
		assert!(Config::is_case_insensitive(&["--insensitive".to_string()]));
	}

	#[test]
	fn invalid_flag()  {
		assert_eq!(
			is_valid_flags(&["-b".to_string()]).err(), 
			Some("Invalid flags: -b".to_string())
		);
	}

	#[test]
	fn valid_flag() {
		assert_eq!(
			is_valid_flags(&["-i".to_string()]).err(), 
			None
		);
	}
}