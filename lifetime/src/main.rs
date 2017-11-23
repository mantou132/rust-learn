use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    // let result; // 变量按创建顺序反向回收
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let string3 = String::from("string3");
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2.as_str(),
        string3.as_str()
    );
    println!("The longest string is {}", result);
}