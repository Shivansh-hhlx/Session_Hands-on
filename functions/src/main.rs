#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


//A function is a block of code for repeating the task to be done multiple times in a code
//For example a code for addition;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

//As per the ppt, 'fn' means we define a function, 'add' is the name of function

fn main() {
    println!("{}", add(5, 4));
    println!("{}", add(5, -4));
    println!("{}", add(10, 2));
    println!("{}", add(-5, -4));

    //Here we can see, we can use the function multiple times with help of its name 'add'
    //If for example we have a 100 line code, we can run it multiple times using functions
    //Variables within normal brackets is data we send and the data after the '->' arrow are what 
    //we get it return
    //Therefore, return value
}
