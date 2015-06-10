// Rust-101, Part 03: Input, Testing
// =================================

// In part 00, I promised that we would eventually replace `read_vec` by a function
// that actually asks the user to enter a bunch of numbers. Unfortunately,
// I/O is a complicated topic, so the code to do that is not pretty - but well,
// let's get that behind us.

// IO/ is provided by the module `std::io`, so we first import that.
// We also import the I/O *prelude*, which brings a bunch of commonly used I/O stuff
// directly available.
use std::io::prelude::*;
use std::io;

// Let's now go over this function line-by-line.
fn read_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    // The central handle to the standard input is made available by `io::stdin()`.
    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D.");
    // We would now like to iterate over standard input line-by-line. We can use a `for` loop
    // for that, but there is a catch: What happens if there is some other piece of code running
    // concurrently, that also reads from standard input? The result would be a mess. Hence
    // Rust requires us to `lock()` standard input if we want to perform large operations on
    // it. (See [the documentation](http://doc.rust-lang.org/stable/std/io/struct.Stdin.html) for more
    // details.) 
    for line in stdin.lock().lines() {
        // The `line` we have here is not yet of type `String`. The problem with I/O is that it can always
        // go wrong, so `line` has type `io::Result<String>`. This is a lot like `Option<String>` ("a `String` or
        // nothing"), but in the case of "nothing", there is additional information about the error.
        // Again, I recommend to check [the documentation](http://doc.rust-lang.org/stable/std/io/type.Result.html).
        // You will see that `io::Result` is actually just an alias for `Result`, so click on that to obtain
        // the list of all constructors and methods of the type.

        // We will be lazy here and just assume that nothing goes wrong: `unwrap()` returns the `String` if there is one,
        // and halts the program (with an appropriate error message) otherwise. Can you find the documentation
        // of `Result::unwrap()`?
        let line = line.unwrap();
        // Now that we have our `String`, we want to make it an `i32`. `parse` is a method on `String` that
        // can convert a string to anything. Try finding it's documentation!

        // In this case, Rust *could* figure out automatically that we need an `i32` (because of the return type
        // of the function), but that's a bit too much magic for my taste. So I use this opportunity to
        // introduce the syntax for explicitly giving the type parameter of a generic function: `parse::<i32>` is `parse`
        // with its generic type set to `i32`.
        match line.parse::<i32>() {
        // `parse` returns again a `Result`, and this time we use a `match` to handle errors (like, the user entering
        // something that is not a number).
        // This is a common pattern in Rust: Operations that could go wrong will return `Option` or `Result`.
        // The only way to get to the value we are interested in is through pattern matching (and through helper functions
        // like `unwrap()`). If we call a function that returns a `Result`, and throw the return value away,
        // the compiler will emit a warning. It is hence impossible for us to *forget* handling an error,
        // or to accidentally use a value that doesn't make any sense because there was an error producing it.
            Ok(num) => vec.push(num),
            Err(_) => println!("What did I say about numbers?"),
        }
    }

    vec
}

// So much for `read_vec`. If there are any questions left, the documentation of the respective function
// should be very helpful. I will not always provide the links, as the documentation is quite easy to navigate
// and you should get used to that.
// 
// The rest of the code dosn't change, so we just copy it.

enum SomethingOrNothing<T>  {
    Something(T),
    Nothing,
}
use self::SomethingOrNothing::{Something,Nothing};

trait Minimum : Copy {
    fn min(a: Self, b: Self) -> Self;
}

fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => T::min(n, e)
        });
    }
    min
}

// `::std::cmp::min` is a way to refer to this function without importing `std`.
// We could also have done `use std::cmp;` and later called `cmp::min`. Try that!
impl Minimum for i32 {
    fn min(a: Self, b: Self) -> Self {
        ::std::cmp::min(a, b)
    }
}

impl SomethingOrNothing<i32> {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

// If you update your `main.rs` to use part 03, `cargo run` should now ask you for some numbers,
// and tell you the minimum. Neat, isn't it?
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}

// After all this nit-picking about I/O details, let me show you quickly something unrelated,
// but really nice: Rust's built-in support for testing.
// Now that the user can run our program on loads of inputs, we better make sure that it is correct.
// To be able to test the result of `vec_min`, we first have to write a function that
// is able to test equality if `SimethingOrNothing`. So let's quickly do that.

// `equals` performs pattern-matching on both `self` and `other` to test the two for being
// equal. Because we are lazy, we want to write only one `match`. so we group the two into a
// pair such that we can match on both of them at once. You can read the first arm of the match
// as testing whether `(self, other)` is `(Nothing, Nothing)`, which is the case exactly if
// both `self` and `other` are `Nothing`. Similar so for the second arm.
impl SomethingOrNothing<i32> {
    fn equals(self, other: Self) -> bool {
        match (self, other) {
            (Nothing     , Nothing     ) => true,
            (Something(n), Something(m)) => n == m,
            // `_` is the syntax for "I don't care", so this is how you add a default case to your `match`.
            _ => false,
        }
    }
}

// Now we are almost done! Writing a test in Rust is shockingly simple. Just write a function
// that takes no arguments as returns nothing, and add `#[test]` right in front of it.
// That's called an *attribute*, and the `test` attribute, well, declares the function to
// be a test.

// Within the function, we can then use `panic!` to indicate test failure. Helpfully, there's
// a macro `assert!` that panics if its argument becomes `false`.
// Using `assert!` and our brand-new `equals`, we can now call `vec_min` with some lists
// and make sure it returns The Right Thing.
#[test]
fn test_vec_min() {
    assert!(vec_min(vec![6,325,33,532,5,7]).equals(Something(5)));
    assert!(vec_min(vec![6,325,33,532]).equals(Something(6)));
}
// To execute the test, run `cargo test`. It should tell you that everything is all right.
// Now that was simple, wasn't it?
// 
// **Exercise**: Add a case to `test_vec_min` that checks the behavior on empty lists.
// 
// **Exercise**: Change `vec_min` such that everything still compiles, but the test fails.
// 
// **Bonus Exercise**: Because `String::parse` is itself generic, you can change `read_vec` to
// be a generic function that works for any type, not just for `i32`. However, you will have to add
// a trait bound to `read_vec`, as not every type supports being parsed. <br/>
// Once you made `vec_min` generic, copy your generic `print` from the previous part. Implement all
// our traits (`Minimum` and `Print`) for `f32` (32-bit floating-point numbers), and change `part_main()`
// such that your program now computes the minimum of a list of floating-point numbers. <br/>
// *Hint*: You can figure out the trait bound `read_vec` needs from the documentation of `String::parse`.
// Furthermore, `std::cmp::min` works not just for `i32`, but also for `f32`.

// [index](main.html) | [previous](part02.html) | next
