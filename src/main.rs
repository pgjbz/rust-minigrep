use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query: &String = &args[1];
    let filename: &String = &args[2];
    println!("Searching {}", query);
    println!("In file {}", filename);

    let content: String = fs::read_to_string(filename).expect("Error on read file");

    println!("File content: \n{}", content);
}
