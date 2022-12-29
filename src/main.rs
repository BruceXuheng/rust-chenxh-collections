
// 所有权
// 所有权是一组规则，用于管理 Rust 程序如何管理内存
// -- Rust 中的每个值都有一个所有者。
// -- 一次只能有一个所有者。
// -- 当所有者超出范围时，将删除该值。
fn main(){
    string_function();
}

// 所有权的 转移和借用
fn string_function(){
    let mut s = String::from("hello");
    s.push_str(",");
    s.push_str("chenxh!");
    let s2 = s;
    println!("{}",s);
}