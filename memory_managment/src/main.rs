fn main() {
    fn1();
    println!("Ownership in rust");
    ownership();
}

fn fn1() {
    let num1 = 32;
    fn2();
    println!("{}", num1);
}

fn fn2() {
    let num3 = 43;
    println!("{}", num3);
}

fn ownership() {
    let s1 = String::from("hello");
    // let s2 = s1;//Shallow copy, when the value owner is changed to s2 and ownership is taken from s1
    // let s2 = s1.clone();
    // println!("{},{}", s1, s2);
    let s2 = ownership_1(s1);
    println!("{}", s2);
}

fn ownership_1(s: String) -> String {
    s
}
