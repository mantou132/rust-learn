use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};

// `stream.lines()` 是一个无限迭代器
// 所以要一个线程来处理
use std::thread;
fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    fs::remove_file("/tmp/rust-learn-socket.sock").expect("file not exist");
    let listener = UnixListener::bind("/tmp/rust-learn-socket.sock").unwrap();

    // async/await?
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}