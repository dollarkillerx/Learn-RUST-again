fn main() {
    println!("Hello, world!");
    test1();
    test2();
    test3();
    test4();
}


// 常规函数 与 关联函数
struct A(i32, i32);

impl A {
    // 关联函数
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // function
    fn math(&self) -> i32 {
        Self::sum(self.0, self.1)
    }
}

fn test1() {
    let b = A(12, 34);
    println!("{}", b.math());
    println!("{}", A::sum(32, 34));

    let a = A(15, 15);
    let add = A::sum;  // A::sum is Fn item type
    let add_math = A::math;  // A::math is Fn item type
    assert_eq!(add(15, 15), A::sum(15, 15));
    assert_eq!(add_math(&a), a.math());
}

// function point , function type
type RGB = (i16, i16, i16);

fn color(_c: &str) -> RGB {
    (23,23,23)
}

fn show(c: fn(&str) -> RGB) {
    println!("{:?}", c("block"));
}

fn test2() {
    let rgb = color;  // rgb: is fn item type
    show(rgb); // reg: is fn pointer type
}

// 闭包
fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    move |n| n + i
}

fn test3() {
    let mut c = counter(32);
    println!("{}", c(23));
}

fn test4() {
    // 1.未被捕获环境变量
    let c1 = || println!("Hello world");
    c1();

    // 可修改环境变量
    let mut arr = [1,23,4];
    let mut c2 = |i| {
      arr[1] = i;
    };
    c2(3);
    println!("{:?}",arr);

    // 未修改环境变量
    let c = "Hello World";
    let c3 = || {
      println!("Hello World {}",c);
    };
    c3();
}

// 1. 如果没有任何捕获变量，则实现FnOnce
// 2. 如果没有捕获变量, 并且会对捕获变量进行修改, 则实现FnMut
// 3. 如果有捕获变量, 并且不会对捕获变量进行修改, 则实现Fn

// # 逃逸闭包
fn c_mut() -> impl Fn(i32) -> i32 {
    let c = 16;
    move |i|c + i
}

fn test5() {
    let mut x = "hello".to_string();
    let mut c = |i: &str| x+=i; // 非逃逸闭包
}