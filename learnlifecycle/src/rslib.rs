mod lifecyc{

    pub fn return_string_ref<'a>(s1: &'a String,s2: & String) -> &'a String{
        if s1.len() > s2.len() {
            s1
        } else {
            println!("{}",s2);
            s1
        }
    }

}

// #[cfg(test)]
// mod tests{
//     use super::lifecyc;
//     #[test]
//     fn test_refrence(){
//         let s1 = String::from("dacy");
//         let s2 = String::from("deigo");
//         let s3 = lifecyc::return_string_ref(&s1,&s2);
//         println!("{},hello!",s3);
//
//
//     }
//
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String>{
        if 2 + 1 == 4{
            Ok(())
        } else {
            Err(String::from("twoo plus two does nto equal for four"))
        }
    }
}