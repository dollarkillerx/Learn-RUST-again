/// # 文档注释
/// 该函数为求和函数
/// # usage:
///    assert_eq!(3, sum(1, 2));
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // 这是单行注释示例
    /*
     * 这是区块注释，被包含的区块 都会被注释
     *
     */

    let x = 5 + /* 90 + */ 5;
    assert_eq!(x, 10);
    println!("2 + 3 = {}", sum(2, 3));
}