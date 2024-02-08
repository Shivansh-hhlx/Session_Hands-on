#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fs::{self, OpenOptions}, io::Write};

fn main() {
    




    // let mut contents = match fs::read("C:/Users/Admin/Desktop/IMP/test.txt") {
    //     Result::Ok(v) => v,
    //     Result::Err(e) => {
    //         println!("Error is ");
    //         dbg!(e);
    //         return
    //     }
    // };
    // dbg!(contents);






    // let stringContent = match fs::read_to_string("C:/Users/Admin/Desktop/IMP/test.txt") {
    //     Result::Ok(s) => s,
    //     Result::Err(e) => {
    //         println!("Error is ");
    //         dbg!(e);
    //         return
    //     }
    // };
    // print!("{}", stringContent);






    // let contents = match fs::read("C:/Users/Admin/Desktop/IMP/test.txt") {
    //     Result::Ok(v) => v,
    //     Result::Err(e) => {
    //         println!("Error is ");
    //         dbg!(e);
    //         return
    //     }
    // };

    // let mut newContents = vec![];

    // for x in contents {
    //     newContents.push(x.wrapping_add_signed(-2));
    // }

    // let mut file = match OpenOptions::new().write(true).open("C:/Users/Admin/Desktop/IMP/test.txt") {
    //     Result::Ok(f) => f,
    //     Result::Err(e) => {
    //         println!("Error is ");
    //         dbg!(e);
    //         return
    //     }
    // };

    // let _ = file.write_all(&newContents);
    // let _ = file.flush();
    // drop(file);


}
