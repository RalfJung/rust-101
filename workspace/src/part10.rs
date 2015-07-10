// Rust-101, Part 10: Closures
// ===========================

use std::fmt;
use part05::BigInt;


// So, let us define a trait that demands that the type provides some method `do_action` on digits.
trait Action {
    fn do_action(&mut self, digit: u64);
}

// Now we can write a function that takes some `a` of a type `A` such that we can call `do_action` on `a`, passing it every digit.
impl BigInt {
    fn act_v1<A: Action>(&self, mut a: A) {
        for digit in self {
            unimplemented!()
        }
    }
}

struct PrintWithString {
    prefix: String,
}

impl Action for PrintWithString {
    // Here we perform performs the actual printing of the prefix and the digit. We're not making use of our ability to
    // change `self` here, but we could replace the prefix if we wanted.
    fn do_action(&mut self, digit: u64) {
        unimplemented!()
    }
}

// Finally, this function takes a `BigInt` and a prefix, and prints the digits with the given prefix.
fn print_with_prefix_v1(b: &BigInt, prefix: String) {
    let my_action = PrintWithString { prefix: prefix };
    b.act_v1(my_action);
}

// Here's a small main function, demonstrating the code above in action. Remember to edit `main.rs` to run it.
pub fn main() {
    let bignum = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    print_with_prefix_v1(&bignum, "Digit: ".to_string());
}

// ## Closures

// This defines `act` very similar to above, but now we demand `A` to be the type of a closure that mutates its borrowed environment,
// takes a digit, and returns nothing.
impl BigInt {
    fn act<A: FnMut(u64)>(&self, mut a: A) {
        for digit in self {
            // We can call closures as if they were functions - but really, what's happening here is translated to essentially what we wrote above, in `act_v1`.
            unimplemented!()
        }
    }
}

// Now that we saw how to write a function that operates on closures, let's see how to write a closure.
pub fn print_with_prefix(b: &BigInt, prefix: String) {
    b.act(|digit| println!("{}{}", prefix, digit) );
}
// You can change `main` to call this function, and you should notice - nothing, no difference in behavior.
// But we wrote much less boilerplate code!

// Remember that we decided to use the `FnMut` trait above? This means our closure could actually mutate its environment.
// For example, we can use that to count the digits as they are printed.
pub fn print_and_count(b: &BigInt) {
    let mut count: usize = 0;
    b.act(|digit| { println!("{}: {}", count, digit); count = count +1; } );
    println!("There are {} digits", count);
}

// ## Fun with iterators and closures

// Let's say we want to write a function that increments every entry of a `Vec` by some number, then looks for numbers larger than some threshold, and prints them.
fn inc_print_even(v: &Vec<i32>, offset: i32, threshold: i32) {
    for i in v.iter().map(|n| *n + offset).filter(|n| *n > threshold) {
        println!("{}", i);
    }
}

// Sometimes it is useful to know both the position of some element in a list, and its value. That's where the `enumerate` function helps.
fn print_enumerated<T: fmt::Display>(v: &Vec<T>) {
    for (i, t) in v.iter().enumerate() {
        println!("Position {}: {}", i, t);
    }
}

// And as a final example, one can also collect all elements of an iterator, and put them, e.g., in a vector.
fn filter_vec_by_divisor(v: &Vec<i32>, divisor: i32) -> Vec<i32> {
    unimplemented!()
}

// **Exercise 10.1**: Look up the [documentation of `Iterator`](http://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) to learn about more functions
// that can act on iterators. Try using some of them. What about a function that sums the even numbers of an iterator? Or a function that computes the
// product of those numbers that sit at odd positions? A function that checks whether a vector contains a certain number? Whether all numbers are
// smaller than some threshold? Be creative!

