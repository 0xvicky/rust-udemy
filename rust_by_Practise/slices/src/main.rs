fn main() {
    //Slices are the reference to the unknown length of Arrays, Vector, or Strings
    println!("Slices!");
    fn1();
    fn2();
}

fn fn1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;
    println!("Success!");
}

fn fn2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice: &[char] = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16); //Here we've used "usize" so the processor architecture decides what to choose for the slice type, so one character use 8 bytes , so both takes 16 bytes

    println!("Success!");
}
