struct User {
    name: String,
    age: u8,
    is_married: bool,
}

fn main() {
    println!("Hello, world!");
    let user_name = input_name();
    let user_age = input_age();
    let user = User {
        name: String::from(user_name),
        age: user_age,
        is_married: false,
    };

    println!("Username:{}", user.name);
    println!("Age:{}", user.age);
    println!("isMarried:{}", user.is_married)
}

fn input_name() -> String {
    println!("Enter username:");
    let mut new_name = String::new();
    std::io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read");
    new_name
}

fn input_age() -> u8 {
    println!("Enter age");
    let mut age = String::new();
    std::io::stdin()
        .read_line(&mut age)
        .expect("Failed to read LIne!");
    let user_age: u8 = age
        .trim()
        .parse()
        .expect("Failed to convert, enter valid number");
    user_age
}
