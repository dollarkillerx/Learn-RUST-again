use std::ops::Add;

fn main() {
    let mut c = [1, 2, 3, 4, 5, 6];
    mdf(&mut c);
    println!("{:?}", c);
    cp(&mut c);
    println!("{:?}", c);

    println!("=================================");
    test1();
    test2();
    test3();
    test4();
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


trait NewAdd<RHS,Output> {
    fn my_add(&self, rhs: RHS) -> Output;
}
impl NewAdd<i32, i32> for i32 {
    fn my_add(&self, rhs: i32) -> i32 {
        self + rhs
    }
}

trait NewAddU32<RHS, Output> {
    fn my_adp(&self, r: RHS) -> Output;
}
impl NewAddU32<i32,u32> for i32{
    fn my_adp(&self,r: i32) -> u32 {
        (self + r) as u32
    }
}

fn test2() {
    let (a,b) = (12i32,34i32);
    println!("{}",a.my_add(12));
    println!("{}",b.my_adp(12));
    let p = "c".to_string();
}

trait MyAdd<RHS = Self> {
    type Output;
    fn my_add(self, rhs:RHS) -> Self::Output;
}
impl MyAdd for u32 {
    type Output = u32;

    fn my_add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

fn test3() {
    let (a, b) = (8u32, 90u32);
    println!("{}",a.my_add(b));
}

#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
fn test4() {
    let p = Point{x: 232,y: -13};
    let p1 = Point{x: 12,y: -123};

    println!("{:?}",p + p1);
}