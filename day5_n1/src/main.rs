fn main() {
    println!("Hello, world!");
    test1();
    test2();
    test3();
    test4();
    test5();
}


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test1() {
    let (a, b) = (23, 34);
    let Point { x, y } = Point { x: 23, y: 23 };
    println!("{} : {}", a, b);
    println!("{} : {}", x, y);
}

fn sum(x: String, ref y: String) -> String {
    x + y
}

fn test2() {
    let s = sum("1.".to_owned(), "2".to_owned());
    println!("{}", s);
}

fn f(x: &Option<String>) {
    match x {
        &Some(ref s) => {
            println!("{}", s)
        }
        &None => {
            println!("nothing!")
        }
    }
}

fn test3() {
    let x = Some("hello".to_owned());
    f(&x);
}

fn test4() {
    let arr = [1, 2, 3];
    match arr {
        [1, _, _] => "starts with one",
        [a, vb, c] => "stats with something else",
    };

    let v = vec![1,2,3];
    match v[..] {
        [a,v] => {},
        [a,c,v] => {println!("{},{},{}",a,c,v)},
        _ => {}, // 必须包含这条分支 因为长度是动态的
    };
}

fn test5() {
    let x: &Option<i32> = &Some(3);
    if let Some(y) = x {
        y; // &i32  当一个引用值以非引用模式匹配 编译器 会自动填写ref 或者 ref mut
    }
}