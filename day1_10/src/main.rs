fn main() {
    println!("Hello, world!");
    test1();
    test2();
}

enum Number {
    One,
    Two,
    Zero,
}
impl Number {
    fn pd(nu: Number) -> String {
        match nu  {
            Number::One => "One".to_string(),
            Number::Two => "Two".to_string(),
            Number::Zero => "Zero".to_string(),
        }
    }
}
fn test1() {
    let c = Number::One;
    println!("{}",Number::pd(c));
}

enum Color {
    Red = 0xff0000,
    Green = 0x000ff00,
    Blue = 0x0000ff,
}
fn test2() {
    println!("#{:06x}",Color::Green as i32);
}

