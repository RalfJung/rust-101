// ***Remember to enable/add this part in `main.rs`!***

// Rust-101, Part 00: Algebraic datatypes
// ======================================

// As our first piece of Rust code, we want to write a function that computes the
// minimum of a list.


// An `enum` for "a number or nothing" could look as follows:
enum NumberOrNothing {
    Number(i32),
    Nothing
}

// Observe how in Rust, the return type comes *after* the arguments.
fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = NumberOrNothing::Nothing;

    // Now we want to *iterate* over the list. Rust has some nice syntax for iterators:
    for el in vec {
        // So `el` is al element of the list. We need to update `min` accordingly, but how do we get the current
        // number in there? This is what pattern matching can do:
        match min {
            // In this case (*arm*) of the `match`, `min` is currently nothing, so let's just make it the number `el`.
            NumberOrNothing::Nothing => {
                unimplemented!()
            },
            // In this arm, `min` is currently the number `n`, so let's compute the new minimum and store it.
            NumberOrNothing::Number(n) => {
                unimplemented!()
            }
        }
    }
    // Finally, we return the result of the computation.
    return min;
}

// Now that we reduced the problem to computing the minimum of two integers, let's do that.
fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        unimplemented!()
    } else {
        unimplemented!()
    }
}

// Phew. We wrote our first Rust function! But all this `NumberOrNothing::` is getting kind of
// ugly. Can't we do that nicer?

// Indeed, we can: The following line tells Rust to take
// the constructors of `NumberOrNothing` into the local namespace.
// Try moving that above the function, and removing all the occurrences `NumberOrNothing::`.
use self::NumberOrNothing::{Number,Nothing};

// To call this function, we now just need a list. Of course, ultimately we want to ask the user for
// a list of numbers, but for now, let's just hard-code something.

fn read_vec() -> Vec<i32> {
    unimplemented!()
}

// Finally, let's call our functions and run the code!
// But, wait, we would like to actually see something, so we need to print the result.

fn print_number_or_nothing(n: NumberOrNothing) {
    unimplemented!()
}

// Putting it all together:
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}

// You can now use `cargo build` to compile your code. If all goes well, try `cargo run` on the
// console to run it.


