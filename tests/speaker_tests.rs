
use crate::speaker;

mod common;

#[test]
fn it_adds_two() {
    speaker::scream();
    common::setup();
    // assert_eq!(4, adder::add_two(2));
}
