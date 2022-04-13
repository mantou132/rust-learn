extern crate hello_macro;
extern crate hello_macro_derive;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec.push(1);
            temp_vec
        }
    };
}



#[derive(HelloMacro)]
struct Pancakes;


fn main() {
    let v: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", v);

    Pancakes::hello_macro();
}
