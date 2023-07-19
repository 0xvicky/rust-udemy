fn main() {
    const PI: f32 = 3.134; //Vale cannot be changed
    println!("{}", PI);

    //OPERATORS
    let is_equal: bool = 3 == 3;
    let is_greater_or_equal: bool = 3 >= 4;
    let is_smaller_or_equal: bool = 3 <= 4;
    let is_not_equal: bool = 3 != 4;
    println!("{}", is_equal); //true
    println!("{}", is_greater_or_equal); //false
    println!("{}", is_smaller_or_equal); //true
    println!("{}", is_not_equal); //true

    //CONDITIONALS
    let flag = false;
    if PI == 3.14 {
        println!("Pi is there!!")
    } else if PI != 3.14 {
        println!("Pi is not thers")
    } else {
        println!("Nothing is there")
    }

    //COOL WAY TO WRITE CONDITIONALS🔥🔥
    let number = if flag { 12 } else { 34 }; //if we initialise a var using conditionals then it is necessary to add "else" statement also
    println!("{}", number);

    //LOOPS
    //Variant: 1
    let mut counter = 0;
    while counter < 10 {
        println!("hello,{}", counter);
        counter += 1;
    }

    //Variant: 2
    let mut counter = 0;
    loop {
        if counter == 4 {
            break;
        } else {
            counter += 1;
            println!("Counter: {}", counter);
        }
    }
}
