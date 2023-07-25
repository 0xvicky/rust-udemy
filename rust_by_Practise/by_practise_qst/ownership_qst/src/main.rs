fn main() {
    println!("Referenece Qstns");
    p1();
    p2();
    p3();
    p4();
    p5();
    p6();
}

fn p1() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

fn p2() {
    let x = 5;
    let y = &x; //& used for reference
                // Modify this line only
    assert_eq!(5, *y); /* * used for dereference */

    println!("Success!");
}

fn p3() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

fn p4() {
    let mut s = String::from("hello, ");

    push_str(&mut s); // Here to change string value, we need to pass mutable reference

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn p5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s; //as to change we have to assign mutable reference

    p.push_str("world");

    println!("Success!");
}

fn p6() {
    // ***
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c; //ref keyword used to take reference of a variable just like &

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
