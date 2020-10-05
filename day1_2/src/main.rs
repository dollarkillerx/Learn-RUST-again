fn main() {
    let a = [1,2,3];
    let b = &a;
    println!("{:p}",&a);
    println!("{:p}",b);

    let mut c = vec![1,2,3];
    let e = &c;
    println!("{:?}",e);
    let d = &mut c;     // 可变借用  
    // let e = &c;      // 借用
    d.push(4);
    d.push(5);
    // println!("{:?}",c);
    // println!("{:?}",e);
    println!("{:?}",d);
}
