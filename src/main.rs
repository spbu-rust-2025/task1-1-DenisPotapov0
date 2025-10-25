
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().expect("Нет первого числа").parse().expect("Некорректное число");
    let b: i32 = parts.next().expect("Нет второго числа").parse().expect("Некорректное число");

    println!("{}", a + b);
}
