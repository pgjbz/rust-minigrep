use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config: Config= Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem on parse arguments: {}", err);
        process::exit(1);
    });

	if let Err(e) = minigrep::is_valid_flags(env::args()) {
		eprintln!("{}", e);
		minigrep::print_flags();
		process::exit(1);
	}

    println!("Searching {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}