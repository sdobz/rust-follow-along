extern crate ch11_tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(common::test_value(), ch11_tests::add_two(2));
}
