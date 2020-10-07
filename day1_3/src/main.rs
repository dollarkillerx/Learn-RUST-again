fn main() {
    let mut c = &scp();
    println!("c: {}",c);
    let p = c;
    println!("p: {}",p);
    // println!("c: {}",c);
}

fn scp() -> String {
    "hello world".to_string()
}
