// https://github.com/rustwasm/team/issues/291

extern {
    fn foo(x: u32) -> u32;
}

#[no_mangle]
pub fn call() -> u32 {
    unsafe {
        foo(42 as u32) as u32
    }
}

#[no_mangle]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {}