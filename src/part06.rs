// Rust-101, Part 06: Lifetimes, Testing
// =====================================

use std::cmp;
use std::ops;
use std::fmt;
use part05::BigInt;


impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        self.data == other.data
    }
}

fn call_eq() {
    let b1 = BigInt::new(13);
    let b2 = BigInt::new(37);
    println!("b1 == b1: {} ; b1 == b2: {}; b1 != b2: {}", b1 == b1, b1 == b2, b1 != b2);
}


impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}



impl BigInt {
    pub fn inc(&mut self, mut by: u64) {
        panic!("Not yet implemented.");
    }
}


#[test]
fn test_inc() {
    let mut b = BigInt::new(1337);
    b.inc(1337);
    assert!(b == BigInt::new(1337 + 1337));

    b = BigInt::new(0);
    assert_eq!(b, BigInt::from_vec(vec![0]));
    b.inc(1 << 63);
    assert_eq!(b, BigInt::from_vec(vec![1 << 63]));
    b.inc(1 << 63);
    assert_eq!(b, BigInt::from_vec(vec![0, 1]));
    b.inc(1 << 63);
    assert_eq!(b, BigInt::from_vec(vec![1 << 63, 1]));
    b.inc(1 << 63);
    assert_eq!(b, BigInt::from_vec(vec![0, 2]));
}

