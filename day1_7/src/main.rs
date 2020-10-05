fn main() {
    test1();
    println!("s1============================");
    test2();
}

fn test1() {
    let mut c = [0;10];
    println!("{}",c.len());
    // c.join(1);
    c[0]= 12;
    println!("{:?}",c);
    for i in c.iter() {
        println!("{}",i);
    }
}

fn test2() {
    let c : [i32;10] = [1,2,3,4,5,6,7,8,9,10];
    let b = &c[1..5];
    println!("{:?}",b);
    let b = vec![1,2,3,4,5];
    let c = &b[..];
    println!("{:?}",c);
}