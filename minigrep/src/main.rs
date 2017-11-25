use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = if let Some(i) = args.get(1) {i} else {"none"};
    let filename = if let Some(i) = args.get(2) {i} else {"none"};

    println!("Searching for {}", query);
    println!("In file {}", filename);
}