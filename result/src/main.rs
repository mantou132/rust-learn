use std::io;
use std::io::Read;
use std::fs::File;

fn main () {
    match read_username_from_file() {
        Ok(string) => println!("{}", string),
        Err(e) => panic!("{:#?}", e)
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}