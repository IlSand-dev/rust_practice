mod vector;

#[derive(Debug)]
struct Value<K, V> {
    key: K,
    value: V,
    next: Option<Box<Value<K, V>>>,
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use vector::vector::MyVector;
fn main() {
    let mut my_vec:MyVector<String> = MyVector::with_capacity(25);
    println!("{}", my_vec.capacity());
    my_vec.push("12".to_string());
    println!("{:?}", my_vec.get(0));
    println!("{:?}", my_vec.get(0));
    let s = "ABCDEFGHIJKLMNOPQRSTIUVWXYZ";
    // for l in s.chars(){
    //     let mut hasher = DefaultHasher::new();
    //     l.hash(&mut hasher);
    //     println!("{}", hasher.finish() & (usize::MAX as u64))
    // }


}
