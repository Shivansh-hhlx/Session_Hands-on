#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


const PI: f64 = 3.1415;

fn main() {
    println!("This is variable");
    let x = 5;
    println!("x is an immutable variable as mut was not explicitly stated {x}");
    // x = 6;
    //If we try to give the variable another value, it fails

    let mut x = 5;
    println!("This is a mutable variable {x}");
    // x = 6;
    // println!("{x}")

    let mut x: u8 = 5;
    println!("This is an explicitly stated u8 variable {x}");
    // x = -1;
    //This will give error because u8 cannot have negative values

    println!("We declared a constant in global scope {PI}");
}
