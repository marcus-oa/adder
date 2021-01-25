use adder;

// Only applicable after changing 'tests/common.rs -> 'tests/common/mod.rs
// Note: Retaining the setup fn from common.rs -> mod.rs
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4,  adder::add_two(2));
}