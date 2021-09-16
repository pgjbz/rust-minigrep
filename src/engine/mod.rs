/// search text case sensitive
/// # Example
/// search("to", "To me for you\nNot me is you");
/// 
/// result -> [];
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	content.lines()
	.filter(|line| line.contains(query))
	.collect()
}

/// search text in content case insenstive
/// # Example
/// search_case_insensitive("to", "To me for you\nNot me is you");
/// 
/// result -> ["To me for you"];
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	content.lines()
	.filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
	.collect()
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
}