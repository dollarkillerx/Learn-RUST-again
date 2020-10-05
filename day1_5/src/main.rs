pub fn calculation(func: fn(i32,i32)->i32) -> i32 {
    let a = 19;
    let b = 10;
    func(a,b)
}

pub fn plus(a: i32,b :i32) -> i32 {
    a + b
}

pub fn less(a: i32,b: i32) -> i32 {
    a - b
}

pub fn x() -> fn(i32,i32) -> i32 {
    less
}

// pub fn x2(x: i32) -> fn(i32,i32) -> i32 {
//     |a: i32,b:i32| -> i32 {
//         a += x;
//         a + b
//     }
// }


pub fn x2c(_x: i32) -> fn(i32,i32) -> i32 {
    fn c(a: i32,b:i32) -> i32 {
        // a += x;  // 注意匿名函数无法捕获上下文的内容
        a + b
    }

    c
}

// 闭包作为参数 传入
pub fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

// 闭包作为返回值
pub fn add_x(i: i32) -> impl Fn(i32) -> i32 {
    // move |c| c + i  简写
    move |c : i32| -> i32 {
        i + c
    }
}

fn main() {
    println!("{}",calculation(plus));
    println!("{}",calculation(less));
    println!("{}",calculation(x()));
    println!("{}",calculation(x2c(23)));

    let a = 12;
    let b = 32;
    println!("{}",math(|| a + b)); // 闭包可以捕获上下文

    println!("{}",add_x(23)(23));
    // println!("{}",calculation(add_x(23)));
}
