struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 12,
    }
}
fn main() {

    let user = build_user(String::from("example@example.com"), String::from("user123"));

    println!("Hello, world!");

}
