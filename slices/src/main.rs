fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5
    println!("'Hello, world!' first word end index is {}", word);
    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    // 字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内
    // 如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出
    let my_string = String::from("hello world");
    // first_word 中传入 `String` 的 slice
    let word2 = first_word2(&my_string[..]);
    print!("'hello world' first word is {}", word2);
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
