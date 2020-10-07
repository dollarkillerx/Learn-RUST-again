fn main() {
    let mut c = [1, 2, 3, 4, 5, 6];
    mdf(&mut c);
    println!("{:?}", c);
    cp(&mut c);
    println!("{:?}", c);

    println!("=================================");
    test1();
}

fn mdf(c: &mut [i32; 6]) {
    c[0] = 23;
}

// 胖指针
fn cp(p: &mut [i32]) {
    // println!("{}",p.len());
    p[3] = 66;
}

struct User {
    name: String,
}

struct Student {
    name: String,
}

trait Ack {
    fn new(name: String) -> Self;
    fn hello(&self) -> String;
}

impl Ack for User {
    fn new(name: String) -> Self {
        User{name}
    }
    fn hello(&self) -> String {
        "User Hello".to_string() + &self.name
    }
}

impl Ack for Student {
    fn new(name: String) -> Self {
        Student{name}
    }
    fn hello(&self) -> String {
        "AcK Hello".to_string() + &self.name
    }
}

fn apc<T: Ack>(name: String) -> T {
    T::new(name)
}

fn test1() {
    let c:User = apc("dkr".to_string());
    let p:Student = apc("stu".to_string());

    println!("{}",c.hello());
    println!("{}",p.hello());
}
