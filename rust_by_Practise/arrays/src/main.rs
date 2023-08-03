fn main() {
    println!("Hello Arrays");
    fn1();
}

fn fn1() {
    let arr1: [&str; 5] = ["hello", "god", "Empathy", "fine", "okokok"];
    println!("{:?}", arr1);
}

fn fn2() {
    // Fill the blank with proper array type
    let arr: [u8; 4] = [1, 2, 3, 4];

    // Modify the code below to make it work
    assert!(arr.len() == 4);

    println!("Success!");
}
