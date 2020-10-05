fn main() {
    test1();
    test2();
    test3();
}

struct Student {
    name: String,
    age: u8,
}
impl Student {
    fn new(name: String,age: u8) -> Self {
        Student{name,age}
    }
    // fn name(&self) -> &String {
    //     // self.name.clone()
    //     &self.name
    // }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn set_name(&mut self,name: String) {
        self.name = name;
    }
}

fn test2() {
    let mut dollar_killer = Student::new("DollarKiller".to_string(), 20);
    println!("{}", dollar_killer.name());
    dollar_killer.set_name("dk".to_string());
    println!("{}", dollar_killer.name());
}

struct Color(i32,i32,i32);
impl Color {
    fn new(x: i32,y: i32,z: i32) -> Self {
        Color(x,y,z)
    }
    fn print(&self) {
        println!("x: {}, y: {}, z: {}",self.0,self.1,self.2);
    }
}

fn test3() {
    let p = Color::new(12,23,34);
    p.print();
}

fn test1() {
    let (a,b) = hello();
    println!("{} : {}",a,b);
    let (_,b) = hello();
    println!("{} : {}",b,b);
    let b = hello();
    println!("{} : {}",b.0,b.1);
}

fn hello() -> (i32,i32) {
    (12,23)
}