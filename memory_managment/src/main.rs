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
    let mut s1 = String::from("hello");
    // let s2 = s1;/s/Shallow copy, when the value owner is changed to s2 and ownership is taken from s1
    // let s2 = s1.clone();
    // println!("{},{}", s1, s2);
    let length = ownership_1(&mut s1);
    println!("The length of {} is {}", s1, length);
    change_name(&mut s1);
    println!("{}", s1);
}

fn ownership_1(s: &str) -> usize {
    s.len() //Here to avoid the double free error , or ownership error, because if we directly pass the string then function param will take the ownerhsip of s1 which again throws error, so we have pass the reference to the s1 to the function param ,so even when the function dropped s1 is still presereved
}

fn change_name(s: &mut String) {
    s.push_str(" Rust");
}
//Mutable Borrowing: When mutable reference is passed, so other variable can manipulate the original value: &mut a
//Read-only Borrowing: when only read only permission is given to the variable by giving &a
