fn main() {
    let options: [&str; 4] = ["Add", "Subtract", "Multiply", "Divide"];
    let mut choice = String::new();

    println!("***********Calculator Project**************");

    //Display options to user
    for (index, operation) in options.iter().enumerate() {
        //enumerate used to get index
        println!("{}. {}", index + 1, operation);
    }
    println!("\nEnter your choice:");

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice = choice.trim().to_string(); //helps to trim the space in input, as it read all the spaces also.

    if choice == "1" {
        println!("Addition is chosed");
    } else if choice == "2" {
        println!("Subtraction is chosen");
    } else if choice == "3" {
        println!("Multiplication is chosed");
    } else {
        println!("Division is selected");
    }
}

// We first show user what operations✅
//Then take user input to select operation✅
//Then again take user input to take two numbers
//The show the result
