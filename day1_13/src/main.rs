struct Dk1;
struct Dk2;

trait Sky {
    fn fly(&self) -> bool;
}

impl Sky for Dk1 {
    fn fly(&self) -> bool {
        true
    }
}

impl Sky for Dk2 {
    fn fly(&self) -> bool {
        false
    }
}

fn static_fn<T: Sky>(f: &T) {
    println!("{}",f.fly());
}

fn dyn_fn(f: &Sky) {
    println!("{}",f.fly());
}

fn main() {
    let d1 = Dk1;
    let d2 = Dk2;
    println!("D1: {}",d1.fly());
    println!("D2: {}",d2.fly());

    // static_fn::<Dk1>(d1);
    // static_fn::<Dk2>(d2);

    static_fn(&d1);
    static_fn(&d2);

    // dyn_fn(&d1);
    // dyn_fn(&d2);
}
