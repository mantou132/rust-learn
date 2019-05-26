#[derive(Default, Debug)]
struct SomeOptions {
    foo: i32,
}

#[derive(Default, Debug)]
struct SomeOptions2 {
    foo: f32,
}

#[derive(Debug)]
enum SomeKind {
    A,
    B,
    C,
}

impl Default for SomeKind {
    fn default() -> Self { SomeKind::A }
}

fn main() {
    let options: SomeOptions = Default::default();
    println!("{:?}", options);

    let options2: SomeOptions2 = Default::default();
    println!("{:?}", options2);

    let kind: SomeKind = Default::default();
    println!("{:?}", kind);
}