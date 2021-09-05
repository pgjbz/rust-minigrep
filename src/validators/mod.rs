const VALID_FLAGS: [&str; 2] = ["-i", "--insensitive"];

pub fn is_valid_flags(args: &[String]) -> Result<(), String> {
		
	let flags: Vec<&String> = args.iter().filter(|arg| arg.starts_with("-")).collect();
	let mut invalid_flags: Vec<String> = Vec::<String>::new();
	for flag in flags {
		if !VALID_FLAGS.contains(&&flag[..]) {
			invalid_flags.push(flag.to_string());
		}
	}
	if invalid_flags.len() > 0 {
		return Err(format!("Invalid flags: {}", invalid_flags.join(", ")));
	}
	Ok(())
}

#[cfg(test)]

mod tests {

	use super::*;

	#[test]
	fn invalid_flag()  {
		assert_eq!(
			is_valid_flags(&["-b".to_string(), "-c".to_string()]).err(), 
			Some("Invalid flags: -b, -c".to_string())
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