extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn stringify_point(point: &Point) {
    let serialized = serde_json::to_string(&point).unwrap();
    println!("{}", serialized);
}

fn parse_point(point_string: &str) {
    // 为什么不是使用 Point 上面的方法？
    let point: Point = serde_json::from_str(point_string).unwrap();
    println!("{:?}", point);
}

fn main() {
    let point = Point { x: 1, y: 2 };
    stringify_point(&point);

    let point_string = r#"
        {
            "x": 3,
            "y": 4
        }
    "#;
    parse_point(point_string);
}
