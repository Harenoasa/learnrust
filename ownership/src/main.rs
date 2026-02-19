use std::io;

fn main() {
    // let s1 = String::from("systemErr");
    // let (s2,len) = calculate_length(s1);
    // println!("the length of \"{}\" is {}",s2,len);

    // let s1 = String::from("systemErr");
    // let len = calculate_length(&s1);
//

    // let mut s = String::from("sample");

    // let r1 = &s;
    // println!("r1 = {}", r1);

    // try_modify_string_ref(&mut s);
    // let r2 = &mut s;
    // println!("s = {}", s);

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{},{}",r1,r2);
    // let s1 = &mut s;
    // // println!("{},{},{}",r1,r2,s1);
    // s1.push_str(", world!");
    // println!("how about original?:{}",s);

    // let r = dangle();
    let mut s = String::new();
    loop {
        s.clear();
        match io::stdin().read_line(&mut s) {
            Ok(n) => {println!("read is successfull, u read :{}",s);break;}
            Err(e) => {println!("read is failed, error is :{}",e);continue;}
        };
    }
    let slice = first_word(&s);
    println!("first word is :{}",slice);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}


// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// fn try_modify_string_ref(s: &mut String ) {
//     s.push_str(" modified");
// }
//
// fn calculate_length(s: &String ) -> usize {
//     s.len()
// }



// fn calculate_length(s:String) -> (String , usize) {
//     let length = s.len();
//     (s, length)
// }