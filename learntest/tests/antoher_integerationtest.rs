use learntest;

mod common;
mod other;


#[test]
fn it_really_adds_two() {
    common::setup();
    assert_eq!(5, learntest::add_two(3))
}

#[test]
fn do_something(){
    assert_eq!(13,other::get_num())
}