fn main() {
    calculator();
}

fn calculator() {
    let options: [&str; 5] = ["Add", "Subtract", "Multiply", "Divide", "Exit"];
    println!("***********Calculator Project**************");
    //Display options to user
    for (index, operation) in options.iter().enumerate() {
        //enumerate used to get index
        println!("{}. {}", index + 1, operation);
    }

    loop {
        let choice = get_input_from_user("Enter your choice");
        if choice == 5 {
            break;
        }
        let num_1 = get_input_from_user("Enter number 1");
        let num_2 = get_input_from_user("Enter number 2");

        match choice {
            1 => println!("Addition: {}", num_1 + num_2),
            2 => {
                if num_1 > num_2 {
                    println!("Subtraction:{}", num_1 - num_2);
                } else {
                    println!("Negatives are not supported");
                }
            }
            3 => println!("Multiplication: {}", num_1 * num_2),
            4 => {
                if num_2 != 0 {
                    println!("Division: {}", num_1 / num_2);
                } else {
                    println!("Cannot divide by zero!");
                }
            }

            _ => println!("Invalid choice. Please enter a number from 1 to 4."),
        }
        println!("*******************END***********************")
    }
}

fn get_input_from_user(prompt: &str) -> u8 {
    let mut input: String = String::new();

    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Failed")
}

// We first show user what operations✅
//Then take user input to select operation✅
//Then again take user input to take two numbers
//The show the result
