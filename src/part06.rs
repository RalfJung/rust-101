// Rust-101, Part 06: Abstract Datastructure, Testing
// ==================================================

use std::cmp;
use std::ops;

pub struct BigInt {
    data: Vec<u64>, // least significant digits first. The last block will *not* be 0.
}

impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }
}

/// Add with carry, returning the sum and the carry
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    match u64::checked_add(a, b) {
        Some(sum) if !carry => (sum, false),
        Some(sum) => { // we have to increment the sum by 1, where it may overflow again
            match u64::checked_add(sum, 1) {
                Some(total_sum) => (total_sum, false),
                None => (0, true) // we overflowed incrementing by 1, so we are just "at the edge"
            }
        },
        None => {
            // Get the remainder, i.e., the wrapping sum. This cannot overflow again by adding just 1, so it is safe
            // to add the carry here.
            let rem = u64::wrapping_add(a, b) + if carry { 1 } else { 0 };
            (rem, true)
        }
    }
}

#[test]
fn test_overflowing_add() {
    assert_eq!(overflowing_add(10, 100, false), (110, false));
    assert_eq!(overflowing_add(10, 100, true), (111, false));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
    assert_eq!(overflowing_add(1 << 63, (1 << 63) -1 , true), (0, true));
}

impl ops::Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        let mut result_vec:Vec<u64> = Vec::with_capacity(cmp::max(self.data.len(), rhs.data.len()));
        let mut carry:bool = false; // the carry bit
        for (i, val) in self.data.into_iter().enumerate() {
            // compute next digit and carry
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(val, rhs_val, carry);
            // store them
            result_vec.push(sum);
            carry = new_carry;
        }
        BigInt { data: result_vec }
    }
}



