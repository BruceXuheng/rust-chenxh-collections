fn main() {
    println!("fucntion add {}", apply(1, 2, function1));
    println!("function multiplication {}", apply(1, 2, function2));
}

fn apply(value: i8, value2: i8, f: fn(i8, i8) -> i8) -> i8 {
    f(value, value2)
}

fn function1(value: i8, value2: i8) -> i8 {
    value + value2
}

fn function2(value: i8, value2: i8) -> i8 {
    value * value2
}
