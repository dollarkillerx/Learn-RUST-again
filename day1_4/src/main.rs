// FizzBuzz    %3 = 0 => fizz  %5 = 0 => buzz  %5 && %3 == 0 => fizzbuzz
use std::sync::Mutex;
use std::collections::HashMap;

pub fn fizz_buzz(num: i32) -> String {
    if num % 5 == 0 && num % 3 == 0 {
        return "fizzbuzz".to_string();
    }else if num % 5 == 0 {
        return "buzz".to_string();
    }else if num % 3 == 0 {
        return "fizz".to_string();
    }

    num.to_string()
}

// lazy_static! {
//     static ref DP: Mutex<HashMap<i32:i32>>
// }

static DP: Mutex<HashMap<i32:i32>>;


fn main() {
    DP = Mutex::new(HashMap::new());
    println!("Hello, world!");
    println!("{}",fizz_buzz(12));


    // let c = HashMap::new();
    
    println!("{}",DP.lock().unwarp().len());
}
