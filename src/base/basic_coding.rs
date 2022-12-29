use std::{cmp::Ordering, fs, io::{self, stdin}};

use rand::Rng;

fn main() {
    // 1、猜数字
    // guess_number();

    // 2、变量和可变性 https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    // variable_function();

    // 3、数据类型
    // data_type();
    array_game_function();


    // 4、函数
    action_fucntion();
    println!("fucntion add {}", apply(1, 2, function1));
    println!("function multiplication {}", apply(1, 2, function2));

    // 5、Control Flow
    control_flow_function();

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

fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop {
        println!("Guess th number?");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Filed to read lien");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("small")
            }
            Ordering::Greater => {
                println!("big")
            }
            Ordering::Equal => {
                println!("Lucky");
                break;
            }
        }
    }
}

// 变量和可变性 演示方法
fn variable_function() {
    let x1 = 5;
    // 变量
    let mut x2 = 5;
    x2 = 7;

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 阴影
    {
        let x1 = 3;
        println!("作用域中:{}", x1);
    }
    println!("作用域外:{}", x1);

    let mut space = "   ";
    let space = space.len();
    println!("阴影 space {}", space);
}

// Rust 有四种主要的标量类型： 整数、浮点数、布尔值和字符
fn data_type() {
    // 整数
    let i1 = b'A';
    let i2 = 8;
    // 浮点
    let i3 = 2.0;
    let i4: f32 = 3.00;

    // remainder
    let remainder = 43 % 5;

    // 布尔
    let i4 = true;

    // 字符
    let c = 'Z';

    // 元组
    let tup1: (i8, f64, u8) = (2, 23.32, 2);
    let tup2 = (1, 2, 3.3, 4.3);
    let (x, y, z, n) = tup2;
    println!("tup2 [2] = {}", y);
    println!("tup2.1 = {}", tup2.1);

    // 阵列类型 - 数组 每个元素都必须具有相同的类型
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组的元素 arr1[0] = {}", arr1[0]);
}

fn array_game_function() {

    let a = [1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is :{element}");

}

// 函数表达式
fn action_fucntion(){
    let y = {
        let x = 8;
        x+1+1
    };
    println!("action fucntion : {y}")
}

// 控制流方法演示
fn control_flow_function() {
    let number = 3;

    // 条件判断
    if number<5{
        println!("if number < 5");
    }else {
        println!("if number > 5");
    }

    let flag = if 1>0 {true} else {false};

    // 循环
    let mut flag = 0;

    println!("loop 开始");
    loop{
        flag = flag+1;
        println!("loop 进行中 flag={flag}");
        if(flag>=5){
            break;
        }
    }
    println!("loop 结束");

    println!("loop 嵌套 开始");
    'loop1break:loop{
        flag = flag+1;
        println!("loop 进行中 flag={flag}");
        loop{
            if(flag>=5){
                break 'loop1break;
            }
        }
    }
    println!("loop 嵌套 结束");

    while flag<8 {
        flag += 1;
        println!("while 进行中 flag={flag}");
    }

    let a = [10, 20, 30, 40, 50];

    for aaa in a {
        println!("a = {aaa}")
    }

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

fn fib_loop(n: u8) {
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
        } else {
        }
    }
}

fn fib_while(n: u8) {
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

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for i in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
}
