use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config= Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem on parse arguments: {}", err);
        process::exit(1);
    });

	if let Err(e) = minigrep::is_valid_flags(&args) {
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