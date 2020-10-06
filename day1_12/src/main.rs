struct User {
    name: String,
    age: u8,
}
impl User {
    fn new(name: String, age: u8) -> Self {
        User{
            name,
            age,
        }
    }
    fn print(&self) {
        println!("name: {}, age: {}",self.name,self.age);
    }
}


fn main() {
    // 分配到堆上
    let dk = Box::new(User::new("dollarKiller".to_string(),18));
    dk.print();

    // 转移到栈上
    let c = *dk;
    c.print();
}
