mod common;
use integration_testing;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, integration_testing::add_two(2));
}
