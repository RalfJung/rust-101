// Rust-101, Part 05: Copy, Clone
// ==============================

use std::cmp;
use std::ops;

// In the course of the next few parts, we are going to build a data-structure for
// computations with *bug* numbers. We would like to not have an upper bound
// to how large these numbers can get, with the memory of the machine being the
// only limit.
// 
// We start by deciding how to represent such big numbers. One possibility here is
// to use a vector of "small" numbers, which we will then consider the "digits"
// of the big number. This is like "1337" being a vector of 4 small numbers (1, 3, 3, 7),
// except that we will use `u64` as type of our base numbers. Now we just have to decide
// the order in which we store numbers. I decided that we will store the least significant
// digit first. This means that "1337" would actually become (7, 3, 3, 1).<br/>
// Finally, we declare that there must not be any trailing zeros (corresponding to
// useless leading zeros in our usual way of writing numbers). This is to ensure that
// the same number can only be stored in one way.

// To write this down in Rust, we use a `struct`, which is a lot like structs in C:
// Just a collection of a bunch of named fields. Every field can be private to the current module
// (which is the default), or public (which would be indicated by a `pub` in front of the name).
// For the sake of the tutorial, we make `dat` public - otherwise, the next parts of this
// course could not work on `BigInt`s. Of course, in a real program, one would make the field
// private to ensure that the invariant (no trailing zeros) is maintained.
pub struct BigInt {
    pub data: Vec<u64>,
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    // Let's start with a constructor, creating a `BigInt` from an ordinary integer.
    // To create an instance of a struct, we write its name followed by a list of
    // fields and initial values assigned to them.
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    // It can often be useful to encode the invariant of a data-structure in code, so here
    // is a check that detects useless trailing zeros.
    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    // We can convert any vector of digits into a number, by removing trailing zeros. The `mut`
    // declaration for `v` here is just like the one in `let mut ...`, it says that we will locally
    // change the vector `v`. In this case, we need to make that annotation to be able to call `pop`
    // on `v`.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        while v.len() > 0 && v[v.len()-1] == 0 {
            v.pop();
        }
        BigInt { data: v }
    }
}

// If you have a close look at the type of `BigInt::from_vec`, you will notice that it
// consumes the vector `v`. The caller hence loses access. There is however something
// we can do if we don't want that to happen: We can explicitly `clone` the vector,
// which means that a full (or *deep*) copy will be performed. Technically,
// `clone` takes a borrowed vector, and returns a fully owned one.
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec(v.clone());
    let b2 = BigInt::from_vec(v);
}

// To be clonable is a property of a type, and as such, naturally expressed with a trait.
// In fact, Rust already comes with a trait `Clone` for exactly this purpose. We can hence
// make our `BigInt` clonable as well.
impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}
// Making a type clonable is such a common exercise that Rust can even help you doing it:
// If you add `#[derive(Clone)]' right in front of the definition of `BigInt`, Rust will
// generate an implementation of `clone` that simply clones all the fields. Try it!
// 
// To put this in perspective, `clone` in Rust corresponds to what people usually manually do in
// the copy constructor of a C++ class: It creates new, independent instance containing the
// same values. Contrary to that, if you pass something to a function normally (like the
// second call to `from_vec` in `clone_demo`), only a *shallow* copy is created: The fields
// are copied, but pointers are simply duplicated. This corresponds to the default copy
// constructor in C++. Rust assumes that after such a copy, the old value is useless
// (as the new one uses the same pointers), and hence considers the data semantically
// moved to the copy. That's another explanation of why Rust does not let you access
// a vector anymore after you passed ownership to some function.

// With `BigInt` being about numbers, we should be able to write a version of `vec_min`
// that computes the minimum of a list of `BigInt`. We start by writing `min` for
// `BigInt`. Now our assumption of having no trailing zeros comes in handy!
impl BigInt {
    fn min(self, other: Self) -> Self {
        // Just to be sure, we first check that both operands actually satisfy our invariant.
        // `debug_assert!` is a macro that checks that its argument (must be of type `bool`)
        // is `true`, and panics otherwise. It gets removed in release builds, which you do with
        // `cargo build --release`.
        // 
        // If you carefully check the type of `BigInt::test_invariant`, you may be surprised that
        // we can call the function this way. Doesn't it take `self` in borrowed form? Indeed,
        // the explicit way to do that would be to call `(&other).test_invariant()`. However, the
        // `self` argument of a method is treated specially by Rust, and borrowing happens automatically here.
        debug_assert!(self.test_invariant() && other.test_invariant());
        // If the lengths of the two numbers differ, we already know which is larger.
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // **Exercise**: Fill in this code.
            panic!("Not yet implemented.");
        }
    }
}

fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
    let mut min: Option<BigInt> = None;
    for e in v {
        // In the loop, `e` now has type `&i32`, so we have to dereference it.
        min = Some(match min {
            None => e.clone(),
            Some(n) => e.clone().min(n)
        });
    }
    min
}
