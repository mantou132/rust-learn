#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(
        Cons(Rc::clone(&value), Rc::new(
            Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    {
        let _c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count of a: {}", Rc::strong_count(&a));
    }
    println!("count of a: {}", Rc::strong_count(&a));


    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
}
