// fn main() {
//     println!("Hello, world!");
//
//     another_function();
// }
//
// fn another_function(){
//     println!("Another function");
// }


// fn main() {
//     println!("Hello, world!");
//
//     another_function(5);
// }
//
// fn another_function(x:i32){
//     println!("The value of x is {x}");
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }
//
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {y}");
// }

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

