fn main() {
    hello1();
    hello2();
}

struct User;
trait Hel {
    fn hello(&self) -> String;
}
trait Hum {
    fn hello(&self);
    fn hello_world<T: Hel>(&self, you: T);
}
struct DK;
impl Hel for DK {
    fn hello(&self) -> String {
        "hello DK".to_string()
    }
}

impl Hum for User {
    fn hello(&self) {
        println!("Hello Hum");
    }

    fn hello_world<T: Hel>(&self, you: T) {
        println!("{}",you.hello());
    }
}

fn hello1() {
    let a = User;
    status_test(a);
}

fn hello2() {
    let a = User;
    dyn_test(&a);
}

fn status_test<T: Hum>(u: T) {
    let b = DK;
    u.hello_world(b);
}

fn dyn_test(c: &dyn Hum) {
    let b = DK;
    c.hello_world(b);
}