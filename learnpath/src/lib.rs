// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() { }
//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
// }


// fn server_order() {
//
// }
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::server_order();
//     }
//     fn cook_order() {}
// }

// mod back_of_house{
//     pub struct BreakFast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//     impl BreakFast {
//         pub fn summer(toast: &str) -> BreakFast {
//             BreakFast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::BreakFast::summer("Rye");
//     meal.toast = String::from("meat");
//     println!("id like {} toast please", meal.toast)
// }


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function(){}
    }
}

use crate::front_of_house::hosting;
// use crate::front_of_house::some_function;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // hosting::some_function();
}



















