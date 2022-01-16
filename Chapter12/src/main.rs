use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searchign for {}",query);
    println!("In file {}",filename);

    let content = fs::read_to_string(filename).expect("Error reading file");

    println!("With text: \n{}",content);
}