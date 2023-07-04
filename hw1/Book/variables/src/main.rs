// fn main() {
//     let mut x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }

// fn main(){
//     let x = 5;
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     println!("The value of x is {x}");
// }

// fn main(){
//     let x = 2.0;
//
//     let y:f32 = 3.0;
// }

// fn main() {
//     let sum = 5 + 10;
//
//     let difference = 95.5 - 4.3;
//
//     let product = 4 * 30;
//
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3;
//
//     let remainder = 43 % 5;
// }

// fn main(){
//     let t = true;
//
//     let f:bool = false;
// }

// fn main(){
//     let c = 'z';
//     let z:char = 'Z';
//     let point_up = '☝';
// }

// fn main(){
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//
//     let (x, y, z) = tup;
//
//     println!("The value of y is: {y}");
//
//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;
// }

// fn main(){
//     let a = [1, 2, 3, 4, 5];
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//         "August", "September", "October", "November", "December"];
//     let first = a[0];
//     let second = a[1];
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
