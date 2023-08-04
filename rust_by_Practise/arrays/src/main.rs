fn main() {
    println!("Hello Arrays");
    fn1();
    fn2();
    fn3();
    fn4();
}

fn fn1() {
    let arr1: [&str; 5] = ["hello", "god", "Empathy", "fine", "okokok"];
    println!("{:?}", arr1);
}

fn fn2() {
    // Fill the blank with proper array type
    let arr: [_; 4] = [1, 2, 3, 4];

    // Modify the code below to make it work
    assert!(arr.len() == 4);

    println!("Success!");
}

fn fn3() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12); //this method prints the space array is taking
    println!("{}", std::mem::size_of_val(&arr0)); //12

    println!("Success!");
}

fn fn4() {
    // Fill the blank with proper array type
    let arr: [usize; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

fn fn5() {
    // Fill the blank
    //shorthand to write 1 one hundred times
    let list: [i32; 100] = [1; 100]; //[1,1,1,1,1,1,1.........100times]

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
