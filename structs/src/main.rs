struct User {
    name: String,
    age: u8,
    is_married: bool,
}

fn main() {
    println!("Hello, world!");

    let user = User {
        name: String::from("Vicky"),
        age: 20,
        is_married: false,
    };

    println!("Username:{}", user.name);
    println!("Age:{}", user.age);
    println!("isMarried:{}", user.is_married)
}
