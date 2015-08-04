use std::ops;
use std::cmp;
use std::fmt;

pub trait Minimum {
    /// Return the smaller of the two
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

/// Return a pointer to the minimal value of `v`.
pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => e.min(n)
        });
    }
    min
}

pub struct BigInt {
    data: Vec<u64>, // least significant digits first. The last block will *not* be 0.
}

// Add with carry, returning the sum and the carry
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let sum = u64::wrapping_add(a, b);
    let carry_n = if carry { 1 } else { 0 };
    if sum >= a { // the first sum did not overflow
        let sum_total = u64::wrapping_add(sum, carry_n);
        let had_overflow = sum_total < sum;
        (sum_total, had_overflow)
    } else { // the first sum did overflow
        // it is impossible for this to overflow again, as we are just adding 0 or 1
        (sum + carry_n, true)
    }
}

// Subtract with carry, returning the difference and the carry
fn overflowing_sub(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let diff = u64::wrapping_sub(a, b);
    let carry_n = if carry { 1 } else { 0 };
    if diff <= a { // the first diff did not wrap
        let diff_total = u64::wrapping_sub(diff, carry_n);
        let had_wrap = diff_total > diff;
        (diff_total, had_wrap)
    } else { // the first diff did wrap
        // it is impossible for this to wrap again, as we are just substracting 0 or 1
        (diff - carry_n, true)
    }
}

impl BigInt {
    /// Construct a BigInt from a "small" one.
    pub fn new(x: u64) -> Self {
        if x == 0 { // take care of our invariant!
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    /// Construct a BigInt from a vector of 64-bit "digits", with the last significant digit being first. Solution to 05.1.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        // remove trailing zeros
        while v.len() > 0 && v[v.len()-1] == 0 {
            v.pop();
        }
        BigInt { data: v }
    }

    /// Increments the number by 1.
    pub fn inc1(&mut self) {
        let mut idx = 0;
        // This loop adds "(1 << idx)". If there is no more carry, we leave.
        while idx < self.data.len() {
            let cur = self.data[idx];
            let sum = u64::wrapping_add(cur, 1);
            self.data[idx] = sum;
            if sum >= cur {
                // No overflow, we are done.
                return;
            } else {
                // We need to go on.
                idx += 1;
            }
        }
        // If we came here, there is a last carry to add
        self.data.push(1);
    }

    /// Increments the number by "by".
    pub fn inc(&mut self, mut by: u64) {
        let mut idx = 0;
        // This loop adds "by * (1 << idx)". Think of "by" as the carry from incrementing the last digit.
        while idx < self.data.len() {
            let cur = self.data[idx];
            let sum = u64::wrapping_add(cur, by);
            self.data[idx] = sum;
            if sum >= cur {
                // No overflow, we are done.
                return;
            } else {
                // We need to add a carry.
                by = 1;
                idx += 1;
            }
        }
        // If we came here, there is a last carry to add
        self.data.push(by);
    }

    /// Return the nth power-of-2 as BigInt
    pub fn power_of_2(mut power: u64) -> BigInt {
        let mut v = Vec::new();
        while power >= 64 {
            v.push(0);
            power -= 64;
        }
        v.push(1 << power);
        BigInt::from_vec(v)
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        self.data == other.data
    }
}

impl Minimum for BigInt {
    // This is essentially the solution to 06.1.
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // compare back-to-front, i.e., most significant digit first
            let mut idx = self.data.len();
            while idx > 0 {
                idx = idx-1;
                if self.data[idx] < other.data[idx] {
                    return self;
                } else if self.data[idx] > other.data[idx] {
                    return other;
                }
            }
            // the two are equal
            return self;
        }
    }
}

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<'a, 'b> ops::Add<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn add(self, rhs: &'a BigInt) -> Self::Output {
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec:Vec<u64> = Vec::with_capacity(max_len);
        let mut carry:bool = false; // the carry bit
        for i in 0..max_len {
            // compute next digit and carry
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(lhs_val, rhs_val, carry);
            // store them
            result_vec.push(sum);
            carry = new_carry;
        }
        if carry {
            result_vec.push(1);
        }
        // We know that the invariant holds: overflowing_add would only return (0, false) if
        // the arguments are (0, 0, false), but we know that in the last iteration, one od the two digits
        // is the last of its number and hence not 0.
        BigInt { data: result_vec }
    }
}

impl<'a> ops::Add<BigInt> for &'a BigInt {
    type Output = BigInt;
    #[inline]
    fn add(self, rhs: BigInt) -> Self::Output {
        self + &rhs
    }
}

impl<'a> ops::Add<&'a BigInt> for BigInt {
    type Output = BigInt;
    #[inline]
    fn add(self, rhs: &'a BigInt) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<BigInt> for BigInt {
    type Output = BigInt;
    #[inline]
    fn add(self, rhs: BigInt) -> Self::Output {
        &self + &rhs
    }
}

impl<'a, 'b> ops::Sub<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn sub(self, rhs: &'a BigInt) -> Self::Output {
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec:Vec<u64> = Vec::with_capacity(max_len);
        let mut carry:bool = false; // the carry bit
        for i in 0..max_len {
            // compute next digit and carry
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_sub(lhs_val, rhs_val, carry);
            // store them
            result_vec.push(sum);
            carry = new_carry;
        }
        if carry {
            panic!("Wrapping subtraction of BigInt");
        }
        // We may have trailing zeroes, so get rid of them
        BigInt::from_vec(result_vec)
    }
}

impl<'a> ops::Sub<BigInt> for &'a BigInt {
    type Output = BigInt;
    #[inline]
    fn sub(self, rhs: BigInt) -> Self::Output {
        self - &rhs
    }
}

impl<'a> ops::Sub<&'a BigInt> for BigInt {
    type Output = BigInt;
    #[inline]
    fn sub(self, rhs: &'a BigInt) -> Self::Output {
        &self - rhs
    }
}

impl ops::Sub<BigInt> for BigInt {
    type Output = BigInt;
    #[inline]
    fn sub(self, rhs: BigInt) -> Self::Output {
        &self - &rhs
    }
}

#[cfg(test)]
mod tests {
    use std::u64;
    use super::{overflowing_add,overflowing_sub,BigInt,Minimum,vec_min};

    #[test]
    fn test_min() {
        let b1 = BigInt::new(1);
        let b2 = BigInt::new(42);
        let b3 = BigInt::from_vec(vec![0, 1]);

        assert_eq!(b1.min(&b2), &b1);
        assert_eq!(b2.min(&b1), &b1);
        assert_eq!(b3.min(&b2), &b2);
        assert_eq!(b2.min(&b3), &b2);
    }

    #[test]
    fn test_vec_min() {
    let b1 = BigInt::new(1);
        let b2 = BigInt::new(42);
        let b3 = BigInt::from_vec(vec![0, 1]);

        assert_eq!(vec_min(&vec![b2.clone(), b1.clone(), b3.clone()]), Some(&b1));
        assert_eq!(vec_min(&vec![b2.clone(), b3.clone()]), Some(&b2));
        assert_eq!(vec_min(&vec![b3.clone()]), Some(&b3));
        assert_eq!(vec_min::<BigInt>(&vec![]), None);
    }

    #[test]
    fn test_overflowing_add() {
        assert_eq!(overflowing_add(10, 100, false), (110, false));
        assert_eq!(overflowing_add(10, 100, true), (111, false));
        assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
        assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
        assert_eq!(overflowing_add(1 << 63, (1 << 63) -1 , true), (0, true));
    }

    #[test]
    fn test_overflowing_sub() {
        assert_eq!(overflowing_sub(100, 10, false), (90, false));
        assert_eq!(overflowing_sub(100, 10, true), (89, false));
        assert_eq!(overflowing_sub(10, 1 << 63, false), ((1 << 63) + 10, true));
        assert_eq!(overflowing_sub(10, 1 << 63, true), ((1 << 63) + 9, true));
        assert_eq!(overflowing_sub(42, 42 , true), (u64::max_value(), true));
    }

    #[test]
    fn test_add() {
        let b1 = BigInt::new(1 << 32);
        let b2 = BigInt::from_vec(vec![0, 1]);
        let b3 = BigInt::from_vec(vec![0, 0, 1]);
        let b4 = BigInt::new(1 << 63);

        assert_eq!(&b1 + &b2, BigInt::from_vec(vec![1 << 32, 1]));
        assert_eq!(&b2 + &b1, BigInt::from_vec(vec![1 << 32, 1]));
        assert_eq!(&b2 + &b3, BigInt::from_vec(vec![0, 1, 1]));
        assert_eq!(&b2 + &b3 + &b4 + &b4, BigInt::from_vec(vec![0, 2, 1]));
        assert_eq!(&b2 + &b4 + &b3 + &b4, BigInt::from_vec(vec![0, 2, 1]));
        assert_eq!(&b4 + &b2 + &b3 + &b4, BigInt::from_vec(vec![0, 2, 1]));
    }

    #[test]
    fn test_sub() {
        let b1 = BigInt::new(1 << 32);
        let b2 = BigInt::from_vec(vec![0, 1]);
        let b3 = BigInt::from_vec(vec![0, 0, 1]);
        let b4 = BigInt::new(1 << 63);

        assert_eq!(&b2 - &b1, BigInt::from_vec(vec![u64::max_value() - (1 << 32) + 1]));
        assert_eq!(&b3 - &b2, BigInt::from_vec(vec![0, u64::max_value(), 0]));
        assert_eq!(&b2 - &b4 - &b4, BigInt::new(0));
        assert_eq!(&b3 - &b2 - &b4 - &b4, BigInt::from_vec(vec![0, u64::max_value() - 1]));
        assert_eq!(&b3 - &b4 - &b2 - &b4, BigInt::from_vec(vec![0, u64::max_value() - 1]));
        assert_eq!(&b3 - &b4 - &b4 - &b2, BigInt::from_vec(vec![0, u64::max_value() - 1]));
    }

    #[test]
    #[should_panic(expected = "Wrapping subtraction of BigInt")]
    fn test_sub_panic1() {
        let _ = BigInt::new(1) - BigInt::new(5);
    }

    #[test]
    #[should_panic(expected = "Wrapping subtraction of BigInt")]
    fn test_sub_panic2() {
        let _ = BigInt::from_vec(vec![5,8,3,33,1<<13,46,1<<49, 1, 583,1<<60,2533]) - BigInt::from_vec(vec![5,8,3,33,1<<13,46,1<<49, 5, 583,1<<60,2533]);
    }

    #[test]
    fn test_inc1() {
        let mut b = BigInt::new(0);
        b.inc1();
        assert_eq!(b, BigInt::new(1));
        b.inc1();
        assert_eq!(b, BigInt::new(2));

        b = BigInt::new(u64::MAX);
        b.inc1();
        assert_eq!(b, BigInt::from_vec(vec![0, 1]));
        b.inc1();
        assert_eq!(b, BigInt::from_vec(vec![1, 1]));
    }

    #[test]
    fn test_power_of_2() {
        assert_eq!(BigInt::power_of_2(0), BigInt::new(1));
        assert_eq!(BigInt::power_of_2(13), BigInt::new(1 << 13));
        assert_eq!(BigInt::power_of_2(64), BigInt::from_vec(vec![0, 1]));
        assert_eq!(BigInt::power_of_2(96), BigInt::from_vec(vec![0, 1 << 32]));
        assert_eq!(BigInt::power_of_2(128), BigInt::from_vec(vec![0, 0, 1]));
    }
}


