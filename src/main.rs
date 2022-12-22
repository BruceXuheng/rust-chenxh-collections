use std::fs;

fn main() {
    println!("fucntion add {}", apply(1, 2, function1));
    println!("function multiplication {}", apply(1, 2, function2));

    let is_pi = pi();
    let is_unit2 = { pi() };

    println!("is_pi:{:?} ,note_pi:{:?}", is_pi, is_unit2);

    let alice = User {
        id: UserId(1),
        name: "name".to_string(),
        gender: Gender::Female,
    };

    println!("alice: {:?}  ", alice);

    // 变量
    // 不可变
    let x = 123;
    // 可变
    let mut y = 123;
    y = 1234;

    // 常量 需要指定类型
    const Y: i32 = 123;

    // 静态变量 需要指定类型
    static X: i32 = 123;
    static mut Z: i32 = 111;

    // 控制流程
    // Rust 支持分支跳转、模式匹配、错误跳转和异步跳转
    // 模拟斐波那契数列
    fib_loop(10);
    fib_while(10);
    fib_for(10);

    // 模式匹配

    // 错误跳转 当错误发生瑞出函数，并返回错误
    // fs::write("test.txt", b"contents")?;
    
    // 异步处理 执行async函数 直到完成，当前上下文可能被阻塞0到多次


}

// 函数的返回值
fn apply(value: i8, value2: i8, f: fn(i8, i8) -> i8) -> i8 {
    f(value, value2)
}

fn function1(value: i8, value2: i8) -> i8 {
    value + value2
}

fn function2(value: i8, value2: i8) -> i8 {
    value * value2
}

fn pi() -> f64 {
    3.1415926
}

fn note_pi() {
    3.1415926;
}

// 结构体
// 结构体有三种形式
// 1.空结构体，不占任何内存空间；
// 2.元组结构体，struct 的每个域都是 匿名的，可以通过索引访问；
// 3.普通结构体，struct 的每个域都有 名字，可以通过名称访问。
#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

// enum
// 1.标签联合。enum可以承载多个不 同的数据结构中的一种。
// 2.枚举
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn fib_loop(n:u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        // 分支跳转
        if i >= n {
            break;
        }else {
            
        }
    }
}

fn fib_while(n:u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
}

fn fib_for(n:u8){
    let (mut a,mut b)= (1,1);
    for i in 2..n{
        let c = a + b;
        a = b;
        b = c;
    }
}