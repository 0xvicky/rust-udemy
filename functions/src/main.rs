fn main() {
    println!("Hello, world!");
    let number = mult(5, 10);
    println!("The number:{}", number); //50
}

fn mult(x: i32, y: i32) -> i32 {
    x * y
}
