//Macros: Macros are the pre-compiled code somewhat similar to functions but they're already compiled so don't need to compile again which ultimately save execution time
use std::io;
macro_rules! hi_macro {
    () => {
        println!("hello Macros")
    };
}

fn main() {
    // println!("My age is :{}", age);
    hi_macro!();

    let age: u8 = 12;
    println!("My age is :{}", age);
    let mut num: String = String::new();
    println!("Input the number to find square:");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let number: u16 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed");
            return;
        }
    };
    sqr(number);
} //Code blocks are the curly brackets which separates the code

fn sqr(num: u16) -> u16 {
    let sqr = num * num;
    println!("Square of {} is {}", num, sqr);
    return sqr;
}
