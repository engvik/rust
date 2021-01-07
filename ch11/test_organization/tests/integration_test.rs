use test_organization;

mod common;

// cargo test integration_test
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}
