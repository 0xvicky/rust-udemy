fn main() {
    println!("Strings");
    str1();
    str2();
    str3();
    str4();
    str5();
    str6();
}

fn str1() {
    //We can't use str type in normal ways, but we can use &str.
    let s: &str = "hello, world"; //can't use str ---> &str ,
    println!("Success!");
    println!("{}", s);
}

fn str2() {
    //into() means to convert the variable type into the declared type
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}
fn greetings(s: &str) {
    println!("{}", s)
}

fn str3() {
    //String type is defined in std and stored as a vector of bytes (Vec), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.
    let mut s = String::new(); //new String initialised in heap
    s.push_str("hello, world"); //If we push a string , we user push_str() method.
    s.push('!'); // But if we only push one character, we use only push() method.
    assert_eq!(s, "hello, world!");
    println!("Success!");
}

fn str4() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats"); //replace used to replace the substring from the existing variable

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

fn str5() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");
    println!("{}", h1);
    println!("Success!");
}

fn str6() {
    //Iterating over the string
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        // Instead of iter() method we use chars() method
        println!("{}", c)
    }
}
