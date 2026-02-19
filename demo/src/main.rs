// struct Rectangle (u32,u32);
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self,other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn create_square(size: u32) -> Rectangle{
        Rectangle{width:size,height:size}
    }
}
fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };
    let mut rect2 = Rectangle { width: 10, height: 40 };
    let mut rect3 = Rectangle { width: 35, height: 55 };
    println!("Area of rectangle1 is {}", rect1.area());
    println!("Area of rectangle2 is {}", rect2.area());
    println!("Area of rectangle3 is {}", rect3.area());
    // println!("can print rec1 again?{:#?}",rect1);
}


// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.0 * rectangle.1
// }
