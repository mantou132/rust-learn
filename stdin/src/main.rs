use std::io::prelude::*;

fn main() {
    println!("What is the quote?");
    // 如果跟后面的 `lock()` 连起来写` io::stdin()` 创建一个临时值资源就会被释放
    // 后面的读取就不能工作
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let quote = lines
        .next()
        .expect("No input")
        .expect("Failed to read")
        .trim()
        .to_string();
    println!("Who said it?");
    let name = lines
        .next()
        .expect("No input")
        .expect("Failed to read")
        .trim()
        .to_string();
    // 要获取所有权才能用 `+` 连接字符串
    println!("{}", name + " says, \"" + &quote + "\"");
}
