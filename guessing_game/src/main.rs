use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    // println!("猜数！");
    // println!("猜测一个数");
    //
    // let secret_number  =rand::thread_rng().gen_range(1,101);
    //
    // loop{
    //     let foo = 1 ;
    //     let bar = foo;
    //     let foo = 2;
    //
    //     let mut guess = String::new();
    //     match io::stdin().read_line(&mut guess) {
    //         Ok(n) => println!("read is successfull, u read :{}",guess),
    //         Err(e) => {println!("read is failed, error is :{}",e);continue;}
    //     };
    //
    //     println!("你猜测的数是: {}", guess);
    //     let guess: u32 = match guess.trim().parse() {
    //         Result::Ok(num ) => num,
    //         Result::Err(_) => {println!("please enter a number!") ;continue; }
    //     };
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         },
    //         Ordering::Greater => println!("You too big!")
    //     }
    // }

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);


}
