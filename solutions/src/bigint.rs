use std::ops;
use std::cmp;
use std::fmt;

pub struct BigInt {
    data: Vec<u64>, // least significant digits first. The last block will *not* be 0.
}

// Add with carry, returning the sum and the carry
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

    /// Construct a BigInt from a vector of 64-bit "digits", with the last significant digit being first
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        // remove trailing zeroes
        while v.len() > 0 && v[v.len()-1] == 0 {
            v.pop();
        }
        BigInt { data: v }
    }

    /// Return the smaller of the two numbers
    pub fn min(self, other: Self) -> Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // compare back-to-front, i.e., most significant digit first
            let mut idx = self.data.len()-1;
            while idx > 0 {
                if self.data[idx] < other.data[idx] {
                    return self;
                } else if self.data[idx] > other.data[idx] {
                    return other;
                }
                else {
                    idx = idx-1;
                }
            }
            // the two are equal
            return self;
        }
    }

    /// Returns a view on the raw digits representing the number.
    /// 
    /// ```
    /// use solutions::bigint::BigInt;
    /// let b = BigInt::new(13);
    /// let d = b.data();
    /// assert_eq!(d, [13]);
    /// ```
    pub fn data(&self) -> &[u64] {
        &self.data[..]
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
        self.data() == other.data()
    }
}

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data().fmt(f)
    }
}

impl<'a, 'b> ops::Add<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn add(self, rhs: &'a BigInt) -> Self::Output {
        let mut result_vec:Vec<u64> = Vec::with_capacity(cmp::max(self.data().len(), rhs.data().len()));
        let mut carry:bool = false; // the carry bit
        for (i, val) in self.data().into_iter().enumerate() {
            // compute next digit and carry
            let rhs_val = if i < rhs.data().len() { rhs.data()[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(*val, rhs_val, carry);
            // store them
            result_vec.push(sum);
            carry = new_carry;
        }
        BigInt::from_vec(result_vec)
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

#[cfg(test)]
mod tests {
    use super::overflowing_add;
    use super::BigInt;

    #[test]
    fn test_overflowing_add() {
        assert_eq!(overflowing_add(10, 100, false), (110, false));
        assert_eq!(overflowing_add(10, 100, true), (111, false));
        assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
        assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
        assert_eq!(overflowing_add(1 << 63, (1 << 63) -1 , true), (0, true));
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


