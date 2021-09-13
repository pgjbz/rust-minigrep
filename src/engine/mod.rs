pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	content.lines()
	.filter(|line| line.contains(query))
	.collect()
}

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