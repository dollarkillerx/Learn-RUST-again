use std::cell::Cell;

#[derive(Debug)]
struct Foo {
    x: u32,
    y: Cell<u32>,
}
fn main() {
    let foo = Foo{x: 1,y: Cell::new(3)};
    println!("{}",foo.y.get());
    foo.y.set(23);
    println!("{}",foo.y.get());
    println!("{:?}",foo);  // y = 32

    let s = "Hello".to_string();
    let bar = Cell::new(s);
    let x = bar.into_inner();
    println!("{}",x);
}
