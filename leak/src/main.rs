use std::{cell::RefCell,rc::Rc};
use std::ops::{Deref, DerefMut};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32 , RefCell<Rc<List>>),
    Nil,
}

impl List {

    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_,item) => Some(item),
            Nil => None,
        }
    }
}
fn test_linked_list(){
    // use std::{cell::RefCell,rc::Rc};
    // let first_node = Rc::new(RefCell::new(Nil));
    // let mut current = first_node.clone();
    //
    // for value in 1..=10 {
    //     let new_node = Rc::new(RefCell::new(Nil));
    //     *current.borrow_mut() = Cons(value,Rc::clone(&new_node));
    //     println!("this node {:?}",current);
    //     current = Rc::clone(&new_node);
    // }
    // println!("first node {:?}",first_node);
    // let mut node = first_node.clone();
    //
    // loop {
    //     let reference = node.borrow();
    //     let next_node = match reference.deref() {
    //         Nil => break,
    //         Cons(value,next_node) => {
    //             println!("the node : {} " , value);
    //             next_node.clone()
    //         },
    //     };
    //     drop(reference);
    //     node = next_node;
    // }
}
fn test_leak(){
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a nex item = {:?}",a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("a next item = {:?}",a.tail());
}
fn main() {
    test_leak();
}
