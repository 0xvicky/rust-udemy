struct User {
    //Basic struct
    name: String,
    age: u8,
    height: f32,
    is_married: bool,
}

impl User {
    //This is how we implement functions in struct
    fn get_name(&self) {
        println!("{}", self.name);
    }
}
fn main() {
    println!("####################Data Structures#####################");

    println!("*****Array******");

    let arr1 = [1, 2, 3, 4, 5, 6];
    println!("{:?}", arr1); //[1, 2, 3, 4, 5, 6]

    let arr2: [i32; 3] = [-4, 2, 6];
    println!("{:?}", arr2); //[-4, 2, 6]
    println!("{}", arr2[1]); //2   //Accessing the elements

    let arr_same = ["Hi"; 10];
    println!("{:?}", arr_same); //["Hi", "Hi", "Hi", "Hi", "Hi", "Hi", "Hi", "Hi", "Hi", "Hi"]

    println!("*******Traversing through Arrays**********");

    let mut index = 0; //Tradition method
    while index < 5 {
        println!("{}", arr1[index]);
        index += 1;
    }

    //For in Loop
    for (index, elem) in arr1.iter().enumerate() {
        println!("{}--->{}", index, elem);
    }

    println!("*******Tuples*********");
    let tuple_1 = (1, 2.3, "3", true); //Tuple is similar to array but it can store different datatypes in them
    println!("{:?}", tuple_1);

    println!("********Vectors**********");
    let mut vector = vec![1, 3, 4, 5, 6];
    vector.push(340); //Vectors are dynamic storage,i.e they stored in heap and follows the rule of ownership
                      //They are very similar to arrays so we can also use for loop to iterate the vector
    println!("{:?}", vector);

    for (index, elem) in vector.iter().enumerate() {
        println!("{}--->{}", index, elem);
    }

    println!("******Structs*******");
    let user = User {
        name: String::from("Vicky"),
        age: 21,
        height: 5.11,
        is_married: false,
    };

    user.get_name();

    println!("********Enums************");
    enum State {
        Locked,
        Unlocked,
        Dead,
    }
    let state = State::Locked;

    match state {
        //match control flow are just like switch statements in other languages
        State::Unlocked => {
            println!("State is unlocked");
        }
        State::Locked => {
            println!("State is locked");
        }
        State::Dead => {
            println!("State is Dead");
        }
    }
}
