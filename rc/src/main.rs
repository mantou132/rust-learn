enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let _a = Rc::new(
        Cons(5, Rc::new(
            Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&_a));
    {
        let _c = Cons(4, Rc::clone(&_a));
        println!("count of a: {}", Rc::strong_count(&_a));
    }
    println!("count of a: {}", Rc::strong_count(&_a));
}
