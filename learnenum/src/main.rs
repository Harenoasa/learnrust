// enum IpAddrKind {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }
// #[derive(Debug)]
// enum Message {
//     Quit ,
//     Move { x :i32,y: i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }
// impl Message {
//     fn call(&self) {
//         println!("{:?}",self);
//     }
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alasaka,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("United States");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quater from {:?}",state);25},
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {

    let v = 0u8;
    // match v {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => (),
    // }

    if let Some(3) = v {
        println!("three");
    }


    // let c = Coin::Quarter((UsState::Alasaka));
    // println!("value_in_cents in : {:?}",value_in_cents(c));




    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(four);
    // route(six);
    // toute(IpAddrKind::V6)

    // let home = IpAddrKind::V4(127,0,0,1);
    // let loopback = IpAddrKind::V6(String::from("::1"));
    //
    // let q = Message:: Quit;
    // let m = Message:: Move {x: 12,y:24};
    // let w = Message:: Write(String::from("hello"));
    // let c = Message:: ChangeColor(0 ,255 ,255);
    //
    // m.call()






}
// fn route(ip_kind: IpAddrKind)  {}
