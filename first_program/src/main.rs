//Macros: Macros are the pre-compiled code somewhat similar to functions but they're already compiled so don't need to compile again which ultimately save execution time

macro_rules! hi_macro {
    () => {
        println!("hello Macros")
    };
}

fn main() {
    println!("Hello, world!");
    hi_macro!()
} //Code blocks are the curly brackets which separates the code
