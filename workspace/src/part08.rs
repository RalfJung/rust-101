// Rust-101, Part 08: Associated Types, Modules
// ============================================

use std::{cmp,ops};
use part05::BigInt;


// So, let us write a function to "add with carry", and give it the appropriate type. Notice Rust's native support for pairs.
fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let sum = u64::wrapping_add(a, b);
    // If an overflow happened, then the sum will be smaller than *both* summands. Without an overflow, of course, it will be
    // at least as large as both of them. So, let's just pick one and check.
    if sum >= a {
        // The addition did not overflow. <br/>
        // **Exercise 08.1**: Write the code to handle adding the carry in this case.
        unimplemented!()
    } else {
        // The addition *did* overflow. It is impossible for the addition of the carry
        // to overflow again, as we are just adding 0 or 1.
        unimplemented!()
    }
}

// `overflow_add` is a sufficiently intricate function that a test case is justified.
// This should also help you to check your solution of the exercise.
/*#[test]*/
fn test_overflowing_add() {
    assert_eq!(overflowing_add(10, 100, false), (110, false));
    assert_eq!(overflowing_add(10, 100, true), (111, false));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
    assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
    assert_eq!(overflowing_add(1 << 63, (1 << 63) -1 , true), (0, true));
}

// ## Associated Types
impl ops::Add<BigInt> for BigInt {

    // Here, we choose the result type to be again `BigInt`.
    type Output = BigInt;

    // Now we can write the actual function performing the addition.
    fn add(self, rhs: BigInt) -> Self::Output {
        // We know that the result will be *at least* as long as the longer of the two operands,
        // so we can create a vector with sufficient capacity to avoid expensive reallocations.
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut result_vec:Vec<u64> = Vec::with_capacity(max_len);
        let mut carry = false; /* the current carry bit */
        for i in 0..max_len {
            // Compute next digit and carry. Store the digit for the result, and the carry for later.
            let lhs_val = if i < self.data.len() { self.data[i] } else { 0 };
            let rhs_val = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            unimplemented!()
        }
        // **Exercise 08.2**: Handle the final `carry`, and return the sum.
        unimplemented!()
    }
}

// ## Traits and borrowed types

// Writing this out becomes a bit tedious, because trait implementations (unlike functions) require full explicit annotation
// of lifetimes. Make sure you understand exactly what the following definition says.
impl<'a, 'b> ops::Add<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn add(self, rhs: &'a BigInt) -> Self::Output {
        // **Exercise 08.3**: Implement this function.
        unimplemented!()
    }
}

// ## Modules

// Rust calls a bunch of definitions that are grouped together a *module*. You can put definitions in a submodule as follows.
mod my_mod {
    type MyType = i32;
    fn my_fun() -> MyType { 42 }
}

// For the purpose of testing, one typically introduces a module called `tests` and tells the compiler
// (by means of the `cfg` attribute) to only compile this module for tests.
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let b1 = BigInt::new(1 << 32);
        let b2 = BigInt::from_vec(vec![0, 1]);

        assert_eq!(&b1 + &b2, BigInt::from_vec(vec![1 << 32, 1]));
        // **Exercise 08.4**: Add some more testcases.
    }
}

