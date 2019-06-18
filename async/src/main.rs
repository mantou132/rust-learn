#![feature(async_await)]

use futures;
use futures::{future, FutureExt};
use std::thread;
use std::time::Duration;

fn long_running_operation(a: u8, b: u8) -> u8 {
    thread::sleep(Duration::from_secs(3));
    println!("long_running_operation");
    a + b
}

fn another_operation(c: u8, d: u8) -> u8 {
    println!("another_operation");
    c * d
}

async fn foo() -> u8 {
    println!("foo");
    let sum = async { long_running_operation(1, 2) };
    let oth = async { another_operation(3, 4) };
    let both = future::join(sum, oth).map(|(sum, _)| sum);
    both.await
}

fn main() {
    let task = foo();
    futures::executor::block_on(async {
        let v = task.await;
        println!("Result: {}", v);
    });
}