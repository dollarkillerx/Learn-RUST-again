fn main() {
    test1();
    test2();
}

fn test1() {
    let dk = User;
    dk.eat();
    dk.read_book();
    dk.lactation();
    dk.sing();
}

struct User;

trait Animal {
    fn eat(&self) {
        println!("eat");
    }
}

trait Primate {
    fn lactation(&self) {
        println!("lactation");
    }
}

trait Human {
    fn sing(&self) {
        println!("sing");
    }
}

trait Students: Human + Primate     // 继承
{
    fn read_book(&self) {
        println!("Read Book"); // 默认实现
    }
}


impl Animal for User {
    fn eat(&self) {
        println!("eat good food"); //重写
    }
}

impl Human for User // 采用默认实现
{}

impl Primate for User // 采用默认实现
{}

impl <T: Human + Primate>Students for T {} // 在不改变User 的时候 让他加上Student的实现689op'

fn test2() {
    let a = Agent;
    test2_dyn(&a);
    test2_static(a);
}

struct Agent;
impl Agent {
    fn hello(&self) {
        println!("Hello");
    }
}
trait Scp {
    fn scp(&self);
}
impl Scp for Agent {
    fn scp(&self) {
        println!("this is scp");
    }
}
fn test2_static<T: Scp>(v: T) {
    v.scp();
}
fn test2_dyn(v: &dyn Scp) {
    v.scp();
}


