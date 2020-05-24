#![no_std]

extern crate alloc;

use alloc::boxed::Box;

#[test]
fn allocate_box() {
    let integer = Box::new(152213u64);
    assert_eq!(*integer, 152213u64)
}