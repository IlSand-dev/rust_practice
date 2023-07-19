use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let vector = vec![1, 2, 3, 4];
    let v:Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vector));
    let a = A{vec: v.clone()};
    let b = A{vec: v.clone()};
    println!("{:?}", a.vec.borrow());
    println!("{:?}", b.vec.borrow());
    a.vec.borrow_mut().push(5);
    println!("{:?}", a.vec.borrow());
    println!("{:?}", b.vec.borrow());
}


struct A{
    vec: Rc<RefCell<Vec<i32>>>
}