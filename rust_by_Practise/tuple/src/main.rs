fn main() {
    //Tuples are the types which can hold many datatypes in single variable
    println!("Tuples!!");
    fn1();
    fn2();
    fn3();
    fn4();
    tuple_fn_arg();
}

fn fn1() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("Success!");
}

fn fn2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "am"); //tuple indexing....
    println!("Success!");
}

fn fn3() {
    //Too long tuple cannot printed, we can only print 12 elemented tuple
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

fn fn4() {
    let tup: (u8, f64, &str) = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

fn tuple_fn_arg() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2i32, 3i32));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
