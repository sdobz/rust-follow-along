extern crate tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(common::test_value(), tests::add_two(2));
}
