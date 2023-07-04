// fn main() {
//     let s = String::from("hello");
//     takes_ownership(s);
//
//     let x = 5;
//     makes_copy(x);
//
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
//
// fn main() {
//     let mut s = String::from("hello");
//
//     change(&mut s);
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    println!("{}",first_word("Hello world!"));
}
