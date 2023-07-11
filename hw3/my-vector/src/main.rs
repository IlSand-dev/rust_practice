mod vector;

use vector::vector::MyVector;
fn main() {
    let mut v = MyVector::new();
    v.push(5);
    v.push(6);
    v.resize(150, 72);
    println!("{:?}", v.get(149));
}
