fn main() {
    //Slices are the reference to the unknown length of Arrays, Vector, or Strings
    println!("Slices!");
    fn1();
}

fn fn1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;

    println!("Success!");
}
