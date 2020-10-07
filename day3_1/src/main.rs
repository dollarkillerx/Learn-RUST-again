
struct DB;
impl DB {
    fn hello(&self) {
        println!("hello");
    }
}
impl Drop for DB {
    fn drop(&mut self) {
        println!("DB Drop");
    }
}

fn main() {
    let d = DB;
    {
        d.hello();
        println!("abc");
    }
    println!("Hello, world!");
}
