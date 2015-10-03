// Rust-101, Part 07: Operator Overloading, Tests, Formatting
// ==========================================================

pub use part05::BigInt;

// With our new knowledge of lifetimes, we are now able to write down the desired type of `min`:
pub trait Minimum {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min: Option<&T> = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => n.min(e)
        });
    }
    min
}

// **Exercise 07.1**: For our `vec_min` to be usable with `BigInt`, you will have to provide an implementation of
// `Minimum`. You should be able to pretty much copy the code you wrote for exercise 06.1. You should *not*
// make any copies of `BigInt`!
impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        unimplemented!()
    }
}

// ## Operator Overloading

impl PartialEq for BigInt {
    #[inline]
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        unimplemented!()
    }
}


// Now we can compare `BigInt`s. Rust treats `PartialEq` special in that it is wired to the operator `==`:
fn compare_big_ints() {
    let b1 = BigInt::new(13);
    let b2 = BigInt::new(37);
    println!("b1 == b1: {} ; b1 == b2: {}; b1 != b2: {}", b1 == b1, b1 == b2, b1 != b2);
}

// ## Testing
// With our equality test written, we are now ready to write our first testcase.
// the `test` attribute. `assert!` is like `debug_assert!`, but does not get compiled away in a release build.
#[test]
fn test_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    unimplemented!()
}
// Now run `cargo test` to execute the test. If you implemented `min` correctly, it should all work!

// ## Formatting

// All formating is handled by [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html). I won't explain
// all the details, and refer you to the documentation instead.
use std::fmt;

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

// Now we are ready to use `assert_eq!` to test `vec_min`.
/*#[test]*/
fn test_vec_min() {
    let b1 = BigInt::new(1);
    let b2 = BigInt::new(42);
    let b3 = BigInt::from_vec(vec![0, 1]);

    let v1 = vec![b2.clone(), b1.clone(), b3.clone()];
    let v2 = vec![b2.clone(), b3.clone()];
    unimplemented!()
}

// **Exercise 07.1**: Add some more testcases. In particular, make sure you test the behavior of
// `vec_min` on an empty vector. Also add tests for `BigInt::from_vec` (in particular, removing
// trailing zeros). Finally, break one of your functions in a subtle way and watch the test fail.
// 
// **Exercise 07.2**: Go back to your good ol' `SomethingOrNothing`, and implement `Display` for it. (This will,
// of course, need a `Display` bound on `T`.) Then you should be able to use them with `println!` just like you do
// with numbers, and get rid of the inherent functions to print `SomethingOrNothing<i32>` and `SomethingOrNothing<f32>`.

