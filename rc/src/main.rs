enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::ops::Deref;
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    //
    // let b = Cons(3,Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    //
    // {
    //     let c = Cons(4,Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after creating c = {}", Rc::strong_count(&a));



    // let a = Rc::new(Cons(5,
    //                      Rc::new(Cons(10,Rc::new(Nil)))));
    // a.clone();

    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));
    // let a = Cons(10, &Nil);
    // let a = Cons(5,
    //              Box::new(Cons(10,
    //              Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(Nil));

    let value1 = Rc::new(RefCell::new(5));
    let value2 = Rc::new(RefCell::new(6));
    let value3 = Rc::new(RefCell::new(7));
    let value4 = Rc::new(RefCell::new(8));
    let value5 = Rc::new(RefCell::new(9));
    let value6 = Rc::new(RefCell::new(10));
    let value7 = Rc::new(RefCell::new(11));
    let a = Rc::new(Cons(Rc::clone(&value1), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::clone(&value2),Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::clone(&value3),Rc::clone(&b)));
    let d = Rc::new(Cons(Rc::clone(&value4),Rc::clone(&c)));
    let e = Rc::new(Cons(Rc::clone(&value5),Rc::clone(&d)));
    let f = Rc::new(Cons(Rc::clone(&value6),Rc::clone(&e)));
    let g = Rc::new(Cons(Rc::clone(&value7),Rc::clone(&f)));
    let mut index = Rc::clone(&g);
    loop{
        match &*index {
            Nil => break,
            Cons(rcrefcell,rclist) => {
                println!("value fron the tail : {}",rcrefcell.borrow());
                index = Rc::clone(rclist);
            },
        }
    }
    println!("The Top of the LinkedList  .");


}
