use std::fs::File;

fn main() {
    println!("Error Handlings");
    let f = File::open("hello.txt").expect("File failed to open");
    let f1 = File::open("hello.txt").unwrap();
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Error occured:{}", err);
    //     }
    // };
}
