# RUST 基础

### 关于所有权和借用
`let key = value;`  // Key: 位置表达式  Value: 值表达式
```rust
fn main() {
    let a = [1,2,3];
    let b = &a;
    println!("{:p}",&a);
    println!("{:p}",b);

    let mut c = vec![1,2,3];
    let d = &mut c;
    d.push(4);
    d.push(5);
    println!("{:?}",c);   // 同一时刻有且仅有一个 可被更改对象
    println!("{:?}",d);
}


fn main() {
    let a = [1,2,3];
    let b = &a;
    println!("{:p}",&a);
    println!("{:p}",b);

    let mut c = vec![1,2,3];
    let e = &c;
    println!("{:?}",e);
    let d = &mut c;     // 可变借用    当存在可变借用   下面的生命周期就不能存在其他借用  
    // let e = &c;      // 借用
    d.push(4);
    d.push(5);
    // println!("{:?}",c);
    // println!("{:?}",e);
    println!("{:?}",d);
}
```

### 思考 
- 全局函数 day1_4
- 函数作为参数返回  day1_5

