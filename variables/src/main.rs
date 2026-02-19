// const MAX_POINTS : u32 = 100_100_120;
fn main() {
    // println!("Hello, world!");
    // let x = 5;
    // let x =x * 2;
    // let x = x + 2;
    // println!("The value of x is {}", x);
    // let spaces = "    ";
    // let spaces = spaces.len();
    //
    // println!("num ; {}", spaces);

    // let guess:u32 = "42".parse().expelet x =

    // let x = 2.checked_ilog10 ()

    // let tu0: (i32, f64 ,u8) = (500 ,6.4,1);
    //
    // let (x,y,z) = tu0;
    // println!("The value of x is {}, y is {} and z is {}", x, y, z)


    // let months = [
    //     "January", "February", "March", "April", "May", "June",
    //     "July", "August", "September", "October", "November", "December"
    // ];
    // let index = [12,13,14,1];
    // println!("value {}",months[index[3]]);
    // println!("Hello, world!");
    // let num:i64 = 10;
    // another_function(num);

    // let x= plus_five(6);
    // println!("x = is {}  ", x);

    // let number = 3;
    //
    // if number < 5 {
    //     println!("condition was true")
    // } else {
    //     println!("condition was false")
    // }

    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4")
    // }else if number % 3 == 0 {
    //     println!("number is divisible by 3")
    // }else if number % 2 == 0 {
    //     println!("number is divisible by 2")
    // }else {
    //     println!("number is not divisible by 4,3, or 2")
    // }

    // let number  = 6;
    // let str = match true{
    //     _ if number % 4 == 0  => "number is divisible by 4",
    //     _ if number % 3 == 0  => "number is divisible by 3",
    //     _ if number % 2 == 0  => "number is divisible by 2",
    //     _ => "number is not divisible by 4,3, or 2"
    // };
    // println!("{}",str);


    // let a = [18,29,38,46,58];
    // for element in a.iter(){
    //     println!("the value is : {}",element)
    // }
    //
    // for number in (1..3).rev() {
    //     println!("the value is : {}",number);
    // }

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 = {}", s1);










    //
    // let condition  =true;
    // let number = if condition {5}else {6};
    // println!("The value of number is {}", number);


    let s =  String::from("hello");;
    let s = take_ownership_and_back(s);;

    let x:i32 = 3;

    makes_copy(x);

    println!("can s be used again? {}",s);

}
fn take_ownership_and_back(some_string : String) -> String {
    println!("{}",some_string);
    some_string
}
fn makes_copy(some_number : i32){
    println!("the number is : {}",some_number)
}




// fn plus_five(x:i32 ) -> i32 {
//     x  +  5
// }
// fn another_function (x: i64){
//     println!("Another functoin:{}",x);
// }


