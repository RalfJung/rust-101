// Rust-101, Part 03: Input
// ========================


// I/O is provided by the module `std::io`, so we first have import that with `use`.
// We also import the I/O *prelude*, which makes a bunch of commonly used I/O stuff
// directly available.
use std::io::prelude::*;
use std::io;

fn read_vec() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::<i32>::new();
    // The central handle to the standard input is made available by the function `io::stdin`.
    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");
    for line in stdin.lock().lines() {
        // Rust's type for (dynamic, growable) strings is `String`. However, our variable `line`
        // here is not yet of that type: It has type `io::Result<String>`.

        // I chose the same name (`line`) for the new variable to ensure that I will never, accidentally,
        // access the "old" `line` again.
        let line = line.unwrap();
        // Now that we have our `String`, we want to make it an `i32`.

        match line.trim().parse::<i32>() {
            Ok(num) => {
                unimplemented!()
            },
            // We don't care about the particular error, so we ignore it with a `_`.
            Err(_) => {
                unimplemented!()
            },
        }
    }

    vec
}


// For the rest of the code, we just re-use part 02 by importing it with `use`.
use part02::{SomethingOrNothing,Something,Nothing,vec_min};

// If you update your `main.rs` to use part 03, `cargo run` should now ask you for some numbers,
// and tell you the minimum. Neat, isn't it?
pub fn main() {
    let vec = read_vec();
    unimplemented!()
}

// **Exercise 03.1**: Define a trait `Print` to write a generic version of `SomethingOrNothing::print`.
// Implement that trait for `i32`, and change the code above to use it.
// I will again provide a skeleton for this solution. It also shows how to attach bounds to generic
// implementations (just compare it to the `impl` block from the previous exercise).
// You can read this as "For all types `T` satisfying the `Print` trait, I provide an implementation
// for `SomethingOrNothing<T>`".
// 
// Notice that I called the function on `SomethingOrNothing` `print2` to disambiguate from the `print` defined previously.
// 
// *Hint*: There is a macro `print!` for printing without appending a newline.
pub trait Print {
    /* Add things here */
}
impl<T: Print> SomethingOrNothing<T> {
    fn print2(self) {
        unimplemented!()
    }
}

// **Exercise 03.2**: Building on exercise 02.2, implement all the things you need on `f32` to make your
// program work with floating-point numbers.

