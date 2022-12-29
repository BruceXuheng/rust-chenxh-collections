use reqwest::header::ValueDrain;

// 所有权
// 所有权是一组规则，用于管理 Rust 程序如何管理内存
// -- Rust 中的每个值都有一个所有者。
// -- 一次只能有一个所有者。
// -- 当所有者超出范围时，将删除该值。
fn main() {
    //
    string_function();

    //
    string_clone();

    // basic_data_function
    basic_data_function();

    //
    make_take_function();

    // 引用可修改
    let mut s = String::from("hello");
    change(&mut s);

    // 借用不可变 就不可再借用可变
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{}, {}", r1, r2);

        // let r3 = &mut s; // BIG PROBLEM

        // println!("{}, {}, and {}", r1, r2, r3);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    {
        let mut s = String::from("hello");
        first_word1(&s);
        println!("--->{}",first_word(&s));
    }
}

// 所有权的 转移和借用
fn string_function() {
    let s1 = String::from("hello");
    let s2 = s1;
    // 所有权 转移给了 s2
    println!("{}, world!", s2);
}

// 数据交互克隆
fn string_clone() {
    let a = String::from("hello");
    let mut b = a.clone();
    b.push_str("b");
    println!("a = {a}, b = {b}");
}

// 栈数据 无需克隆
fn basic_data_function() {
    let a = 1;
    let b = a;
    println!("a = {a}, b = {b}");
}

// 变量引用 所有权转移
fn make_take_function() {
    let mut s = String::from("这是中文");
    take_ownership(&s);
    println!("make_take_function: {s}");
    s = takes_and_gives_back(s);

    let s1 = String::from("hello");
    // s1所有权丢失
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

// 只能是引用。如果不是引用直接 传递所有权也会传递
fn take_ownership(s: &String) {
    println!("take_ownership: {s}")
}

// 返回值返回所有权
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// 引用 示例
// fn main() {
//     let str1 = String::from("Hello");
//     let str1_length = calculate_length1(&str1);

// }

// fn calculate_length1(s:&String)->usize{
//     s.len()
// }

// 引用并修改
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 悬空指针错误示例
// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("ele[i] = {}", i);
        println!("ele[item] = {}", &item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word1(str1: &String) -> &str {
    let bytes = str1.as_bytes();
    let mut result = String::from("");
    for (index, &value) in bytes.iter().enumerate() {
        println!("ele[0] = {}", index);
        println!("ele[1] = {}", &str1[..index]);
        if index == 1 {
            // result = value;
            return &str1[..index];
        }
    }

    "123"
}
