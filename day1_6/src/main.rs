fn main() {
    test1();
    println!("=====================s1");
    test2();
    println!("=====================s2");
    // test3();
    test4();
}

fn test1() {
    let a = 12;
    let b = 32;
    let c = if a > b {a}else{b};
    println!("{}",c);
}

fn test2() {
    for i in 1..30 {  // 前闭后开
        if i % 3 == 0 {
            println!("fizz");
        }else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}",i.to_string());
        }
    }
}

fn test3() {
    for i in 0..30 {
        match i {
            // i > 0 => {
            //     println!("fizzBuzz");
            // }
            // i % 3 == 0 => {
            //     println!("fizz");
            // }
            // i % 5 == 0 => {
            //     println!("buzz");
            // }
            _ => {
                println!("{}",i.to_string());
            }
        }
    }
}

fn test4() {
    let mut c = vec![1,2,3,4,5,6,6,7,8,8,2,4,2,342,234,32,4];
    while let Some(x) = c.pop() {
        println!("{}",x);
    }
}