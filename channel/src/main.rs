use std::{sync::mpsc, thread, time::Duration};

fn return_index<'a>(index_a: &'a str, index_b: &str) -> &'a str {
    //    if index_a.chars().nth(0) == Some('a') {
    //        return index_a;
    //    }
    index_a
}
fn lifecycle_test() {
    let string_a = String::from("xihuan chang tiao rap");
    let index: &str;
    {
        let string_b = String::from("shikun");
        index = return_index(&string_a, &string_b);
    }
    println!("can you use the index ? :{}", index);
}
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let mut count_loop = 0;
    let received;
    loop {
        received = match rx.try_recv() {
            Err(_) => {
                println!("failed. retrying...");
                count_loop = count_loop + 1;
                if count_loop == 4 {
                    panic!("out of try limit")
                };
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            Ok(string) => string,
        };
        break;
    }
    println!("Got: {}", received);
    //
    //     lifecycle_test();
}
