fn main() {
    test1();
    test2();
}

fn test2() {
    let mut a = 12;
    let ptr_a = &mut a as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
        *ptr_a += *ptr_y;
    }
    println!("{}",a);
}

fn test1() {
    let s: &'static str = "wo shi ni ba ba";
    let p = s.as_ptr();
    println!("{:?}",p);
    let len = s.len();

    let s = unsafe {
        let slice = std::slice::from_raw_parts(p,len);
        std::str::from_utf8(slice)
    };

    println!("{:?}",s);
}
