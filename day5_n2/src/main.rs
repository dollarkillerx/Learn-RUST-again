fn main() {
    // 智能指针复习
    test1();
    test2();
    // 智能指针复习 End
}

fn test1() {
    let x: Box<i32> = Box::new("43".parse().unwrap());
    let y = *x;
    assert_eq!(y, 43)
}

fn test2() {
    let wang = User {
        name: "你大爷的".to_string(),
    };
    println!("{:?}", wang);
}

#[derive(Debug)]
struct User {
    name: String,
}

impl Drop for User {
    fn drop(&mut self) {
        println!("{} Drop", &self.name)
    }
}

fn test3() {

}