### Trait
孤儿规则:
要实现某个trait必须是 要实现的trait或者要实现trait的类型 在当前的crate中
比如
```rust
use std::ops::Add;
impl Add<u32> for i32 {
    type Output = u32;
    fn add(self, r: u32) -> Output {
        (self as u32) + u32
    }
}
```
当前代码就会违反孤儿规则  当前crate 没有trait Add 也没有类型i32