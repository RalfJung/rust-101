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


// Add with carry, returning the sum and the carry
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let sum = u64::wrapping_add(a, b);
    if sum >= a {
        panic!("First addition did not overflow. Not implemented.");
    } else {
        panic!("First addition *did* overflow. Not implemented.");
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
        panic!("Not yet implemented.");
    }
}
