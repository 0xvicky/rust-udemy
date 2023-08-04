fn main() {
    println!("STructs");
    fn1();
    fn2();
    fn3();
    fn4();
}

fn fn1() {
    #[derive(Debug)]
    struct User {
        age: u32,
        isRust: bool,
    }

    let struct1: User = User {
        age: 40u32,
        isRust: true,
    };

    let struct2: User = User {
        age: 23u32,
        ..struct1
    };

    println!("{:?}", struct1);
    println!("{:?}", struct2);
}

struct Person {
    name: String,
    age: u8,
}

fn fn2() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn fn3() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

fn fn4() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    assert_eq!(s, "hello, world!");
    move_ownership(s);

    println!("Success!");
}
fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
