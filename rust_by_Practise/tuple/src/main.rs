fn main() {
    //Tuples are the types which can hold many datatypes in single variable
    println!("Tuples!!");
    fn1();
    fn2();
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
