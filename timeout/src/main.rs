use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    let now = SystemTime::now();

    // we sleep for 2 seconds
    sleep(Duration::new(2, 1));
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("secs: {}", elapsed.as_secs());
            println!("nanos: {}", elapsed.subsec_nanos());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
}
