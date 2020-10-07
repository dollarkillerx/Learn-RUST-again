fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

// vec 向量
fn test1() {
    let mut c = vec![10;1];
    c.push(1);
    c.push(2);
    c.push(3);
    c.push(4);
    while let Some(a) = c.pop() {
        println!("a: {}",a);
    }
}


// 双端链表
use std::collections::VecDeque;
fn test2() {
    let mut c = VecDeque::new();

    c.push_back(1); // append
    c.push_back(1); // append
    c.push_back(1); // append
    println!("{}",c.pop_back().unwrap()); // pop

    c.push_front(2); // header
    c.push_front(2); // header
    c.push_front(2); // header
    println!("{}",c.pop_front().unwrap());
}

// LinkedList   语法与VecDeque 相同 但是 存储在栈上
use std::collections::LinkedList;
fn test3() {
    let mut c= LinkedList::new();

    c.push_back(1); // append
    c.push_back(1); // append
    c.push_back(1); // append
    println!("{}",c.pop_back().unwrap()); // pop

    c.push_front(2); // header
    c.push_front(2); // header
    c.push_front(2); // header
    println!("{}",c.pop_front().unwrap());
}

// HashMap ,BTreeMap
use std::collections::HashMap;
use std::collections::BTreeMap;
fn test4() {
    let mut h_map = HashMap::new();
    let mut b_map = BTreeMap::new();

    h_map.insert(1,"a");
    h_map.insert(2,"b");
    h_map.insert(3,"c");

    b_map.insert(1,"a");
    b_map.insert(2,"a");
    b_map.insert(3,"a");

    println!("h map {:?}",h_map);
    println!("b map {:?}",b_map);
}

// HSet BTreeSet
use std::collections::HashSet;
use std::collections::BTreeSet;
fn test5() {
    let mut h_set = HashSet::new();
    let mut b_set = BTreeSet::new();

    h_set.insert("Hello");
    h_set.insert("World");
    h_set.insert("GOD");

    b_set.insert("Hello");
    b_set.insert("World");
    b_set.insert("GOD");

    println!("{:?}",h_set);
    println!("{:?}",b_set);
}