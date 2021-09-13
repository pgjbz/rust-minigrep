use std::env::Args;

const VALID_FLAGS: [&str; 2] = ["-i", "--insensitive"];

pub fn is_valid_flags(args: Args) -> Result<(), String> {
		
	let flags: Vec<String> = args.into_iter().filter(|arg| arg.starts_with("-")).collect();
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

pub fn print_flags() {
	eprintln!("
	Valid flags:
	-i, --insensitive\t\tCase insensitive flag
	")
}
