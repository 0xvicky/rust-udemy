fn main() {
    println!("Box");

    let x: Box<i32> = Box::new(5); //Box is a concept which allows to allocate any stacked datatype directly into the heap and return the address in heap.

    let mut y: Box<i32> = Box::new(1);

    *y = 4; //To get the value that is stored in that address we can dereferenece using * to get that value
    println!("{}", x);
    assert_eq!(*x, 5);

    println!("Success!");
}
