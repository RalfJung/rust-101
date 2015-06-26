// ***Remember to enable/add this part in `main.rs`!***

// Rust-101, Part 08: Associated Types, Modules (WIP)
// ==================================================

use std::cmp;
use std::ops;
use std::fmt;
use part05::BigInt;

// Add with carry, returning the sum and the carry
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let sum = u64::wrapping_add(a, b);
    if sum >= a { // first addition did not overflow
        unimplemented!()
    } else { // first addition *did* overflow
        unimplemented!()
    }
}

/*#[test]*/
fn test_overflowing_add() {
    assert_eq!(overflowing_add(10, 100, false), (110, false));
    assert_eq!(overflowing_add(10, 100, true), (111, false));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
    assert_eq!(overflowing_add(1 << 63, (1 << 63) -1 , true), (0, true));
}

impl ops::Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        let mut result_vec:Vec<u64> = Vec::with_capacity(cmp::max(self.data.len(), rhs.data.len()));
        unimplemented!()
    }
}

// [index](main.html) | [previous](part07.html) | [next](main.html)
