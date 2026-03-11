use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("val is {}", val);
        let val = String::from("shikund@outlook.com");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {}", received);
}
