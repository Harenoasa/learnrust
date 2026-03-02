mod rslib;

#[derive(Debug)]
pub struct Rectangle{
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }
    pub fn can_hold(&self,rec: &Rectangle) -> bool {
        self.width > rec.width && self.length > rec.length ||
            self.width > rec.length && self.length > rec.width
    }
}
pub fn greeting(name :&str) -> String {
    format!("hallo ! {} ",name)
}

struct Guess {
    value: u32,
}

fn create_guess(value: u32) -> Guess{

    if(value > 100 ||value < 1){
        panic!("The value should be less than 100 and greater than 0");
    }


    Guess { value }
}



mod tests {


    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         length: 8,
    //         width: 7,
    //     };
    //     let smaller = Rectangle {
    //         length: 5,
    //         width: 1,
    //     };
    //     // assert!(larger.can_hold(&smaller));
    //     assert!(!smaller.can_hold(&larger),"can contain the rectangle");
    //
    // }
    // #[test]
    // fn exploration(){
    //     assert_eq!(2*2,4);
    //     // println!("Hello, worlddd!");
    //     panic!("test test failed");
    // }

    // use super::greeting;
    // #[test]
    // fn greetings_contain_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"));
    // }

    use super::Guess;
    use super::create_guess;

    #[test]
    #[should_panic(expected = "")]
    fn greater_than_100(){
        create_guess(111);
    }



}