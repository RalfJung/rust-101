// Rust-101, Part 05: Clone, Copy
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

// We can also make the type `SomethingOrNothing<T>` implement `Clone`. However, that
// can only work if `T` is `Clone`! So we have to add this bound to `T` when we introduce
// the type variable.
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            // In the second arm of the match, we need to talk about the value `v`
            // that's stored in `self`. However, if we would write the pattern as
            // `Something(v)`, that would indicate that we *own* `v` in the code
            // after the arrow. That can't work though, we have to leave `v` owned by
            // whoever called us - after all, we don't even own `self`, we just borrowed it.
            // By writing `Something(ref v)`, we just borrow `v` for the duration of the match
            // arm. That's good enough for cloning it.
            Something(ref v) => Something(v.clone()),
        }
    }
}
// Again, Rust will generate this implementation automatically if you add
// `#[derive(Clone)]` right before the definition of `SomethingOrNothing`.

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
            // **Exercise 05.1**: Fill in this code.
            panic!("Not yet implemented.");
        }
    }
}

// Now we can write `vec_min`. In order to make it type-check, we have to write it as follows.
fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
    let mut min: Option<BigInt> = None;
    for e in v {
        min = Some(match min {
            None => e.clone(),
            Some(n) => e.clone().min(n)
        });
    }
    min
}
// Now, what's happening here? Why do we have to write `clone()`, and why did we not
// have to write that in our previous version?
// 
// The answer is already hidden in the type of `vec_min`: `v` is just borrowed, but
// the Option<BigInt> that it returns is *owned*. We can't just return one
// of the elements of `v`, as that would mean that it is no longer in the vector!
// In our code, this comes up when we update the intermediate variable `min`, which
// also has type `Option<BigInt>`. If you replace `e.clone()` in the `None` arm
// with `*e`, Rust will complain "Cannot move out of borrowed content". That's because
// `e` is a `&BigInt`. Assigning `min` to `*e` works just like a function call:
// Ownership of the underlying data (in this case, the digits) is transferred from
// the vector to `min`. But that's not allowed, since we must retain the vector
// in its existing state. After cloning `e`, we own the copy that was created,
// and hence we can store it in `min`.<br/>
// Of course, making such a full copy is expensive, so we'd like to avoid it.
// That's going to happen in the next part.
// 
// But before we go there, I should answer the second question I brought up above:
// Why did our old `vec_min` work? We stored the minimal `i32` locally without
// cloning, and Rust did not complain. That's because there isn't really much
// of an "ownership" when it comes to types like `i32` or `bool`: If you move
// the value from one place to another, then both instance are independent
// and complete instances of their type. This is in stark contrast to types
// like `Vec<i32>`, where merely moving the value results in both the old
// and the new vector to point to the same underlying buffer.
//
// Rust calls types like `i32` that can be freely duplicated `Copy` types.
// `Copy` is another trait, and it is implemented for the basic types of
// the language. Remember how we defined the trait `Minimum` by writing
// `trait Minimum : Copy { ...`? This tells Rust that every type that
// implements `Minimum` must also implement `Copy`, and that's why Rust
// accepted our generic `vec_min` in part 02.
// 
// Curiously, `Copy` is a trait that does not require any method to
// be implemented. Implementing `Copy` is merely a semantic statement,
// saying that the idea of ownership does not really apply to this type.
// Traits without methods are called *marker traits*. We will see some
// more examples of such traits later.
// 
// If you try to implement `Copy` for `BigInt`, you will notice that Rust
// does not let you do that. A type can only be `Copy` if all its elements
// are `Copy`, and that's not the case for `BigInt`. However, we can make
// `SomethingOrNothing<T>` copy if `T` is `Copy`.
impl<T: Copy> Copy for SomethingOrNothing<T>{}
// Again, Rust can generate implementations of `Copy` automatically. If
// you add `#[derive(Copy,Clone)]` right before the definition of `SomethingOrNothing`,
// both `Copy` and `Clone` will automatically be implemented.

// In closing this part, I'd like to give you another perspective on the
// move semantics (i.e., ownership passing) that Rust applies, and how
// `Copy` and `Clone` fit.<br/>
// When Rust code is executed, passing a value (like `i32` or `Vec<i32>`)
// to a function will always result in a shallow copy being performed: Rust
// just copies the bytes representing that value, and considers itself done.
// That's just like the default copy constructor in C++. Rust, however, will
// consider this a destructive operation: After copying the bytes elsewhere,
// the original value must no longer be used. After all, the two could not
// share a pointer! If, however, you mark a type `Copy`, then Rust will *not*
// consider a move destructive, and just like in C++, the old and new value
// can happily coexist. Now, Rust does not allow to to overload the copy
// constructor. This means that passing a value around will always be a fast
// operation, no allocation or copying of large data of the heap will happen.
// In the situations where you would write a copy constructor in C++ (and hence
// incur a hidden cost on every copy of this type), you'd have the type *not*
// implement `Copy`, but only `Clone`. This makes the cost explicit.
