fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Hello, world! {}", s1);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

    println!("emoji {}", "\u{1f60a} 😊 \"");
    println!("space byte is {}", b' ');
}
