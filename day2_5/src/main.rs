use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;

trait Fly {
    fn fly(&self) -> bool;
}
impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}
impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

fn fly_static(f: impl Fly + Debug) -> bool {
    f.fly()
}
fn can_fly(f: impl Fly + Debug) -> impl Fly {
    if f.fly() {
        println!("can {:?}",f);
    }else {
        println!("not can {:?}",f);
    }
    f
}



fn main() {
    let a = Pig;
    let b = Duck;

    let pig = can_fly(a);
    let duck = can_fly(b);

    test1();
}

fn test1() {
    let a = "s1".to_string();

    let b = "s2".to_string();
    // let c = a + &b;
    let c = a.add(&b);
    println!("c: {}",c);
    println!("b: {}",b);
}
