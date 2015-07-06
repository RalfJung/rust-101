// Rust-101, Part 09: Iterators (WIP)
// ==================================

use part05::BigInt;

// In the following, we will look into the iterator mechanism of Rust and make our `BigInt` compatible
// with the `for` loops. Of course, this is all about implementing particular traits again. In particular,
// an iterator is something that implements the `Iterator` trait. As you can see in [the documentation](http://doc.rust-lang.org/beta/std/iter/trait.Iterator.html),
// this trait mandates a single function `next` returning an `Option<Self::Item>`, where `Item` is an
// associated type chosen by the implementation. (There are many more methods provided for `Iterator`,
// but they all have default implementations, so we don't have to worry about them right now).
// 
// For the case of `BigInt`, we want our iterator to iterate over the digits in normal, notational order: The most-significant
// digit comes first. So, we have to write down some type, and implement `Iterator` for it such that `next` returns the digits
// one-by-one. Clearly, the iterator must somehow be able to access the number it iterates over, and it must store its current
// location. However, it cannot *own* the `BigInt`, because then the number would be gone after iteration! That'd certainly be bad.
// The only alternative is for the iterator to *borrow* the number.

// In writing this down, we again have to be explicit about the lifetime of the borrow: We can't just have an
// `Iter`, we must have an `Iter<'a>` that borrowed the number for lifetime `'a`. <br/>
// `usize` here is the type of unsigned, pointer-sized numbers. It is typically the type of "lengths of things",
// in particular, it is the type of the length of a `Vec` and hence the right type to store an offset into the vector of digits.
struct Iter<'a> {
    num: &'a BigInt,
    idx: usize, // the index of the last number that was returned
}

// Now we are equipped to implement `Iterator` for `Iter`.
impl<'a> Iterator for Iter<'a> {
    // We choose the type of things that we iterate over to be the type of digits, i.e., `u64`.
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // First, check whether there's any more digits to return.
        if self.idx == 0 {
            // We already returned all the digits.
            unimplemented!()
        } else {
            // Decrement, and return next digit.
            unimplemented!()
        }
    }
}

// All we need now is a function that creates such an iterator for a given `BigInt`.
impl BigInt {
    // Notice that when we write the type of `iter`, we don't actually have to give the lifetime parameter of `Iter`. Just as it is
    // the case with functions returning borrowed data, you can elide the lifetime. The rules for adding the lifetimes are exactly the
    // same. (See the last section of [part 06](part06.html).)
    fn iter(&self) -> Iter {
        unimplemented!()
    }
}

// We are finally ready to iterate! Remember to edit `main.rs` to run this function.
pub fn main() {
    let b = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    for digit in b.iter() {
        println!("{}", digit);
    }
}

