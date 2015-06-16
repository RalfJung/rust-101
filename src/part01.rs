// Rust-101, Part 01: Expressions, Inherent methods
// ================================================

use std;

// Even though our code from the first part works, we can still learn a
// lot by making it prettier. To understand how, it is important to
// understand that Rust is an "expression-based" language. This means that most of the
// terms you write down are not just *statements* (executing code), but *expressions*
// (returning a value). This applies even to the body of entire functions!

// For example, consider `sqr`:
fn sqr(i: i32) -> i32 { i * i }
// Between the curly braces, we are giving the *expression* that computes the return value.
// So we can just write `i * i`, the expression that returns the square if `i`!
// This is very close to how mathematicians write down functions (but with more types).

// Conditionals are also just expressions. You can compare this to the ternary `? :` operator
// from languages like C.
fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

// And the same applies to case distinction with `match`: Every `arm` of the match
// gives the expression that is returned in the respective case.
// (We repeat the definition from the previous part here.)
enum NumberOrNothing {
    Number(i32),
    Nothing
}
use self::NumberOrNothing::{Number,Nothing};
fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
    match n {
        Nothing => default,
        Number(n) => n,
    }
}

// With this fresh knowledge, let us now refactor `vec_min`.
fn vec_min(v: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;
    for e in v {
        // Notice that all we do here is compute a new value for `min`, and that it will always end
        // up being a `Number` rather than `Nothing`. In Rust, the structure of the code
        // can express this uniformity.
        min = Number(match min {
            Nothing => e,
            Number(n) => std::cmp::min(n, e)
        });
    }
    // The `return` keyword exists in Rust, but it is rarely used. Instead, we typically
    // make use of the fact that the entire function body is an expression, so we can just
    // write down the desired return value.
    min
}

// Now that's already much shorter! Make sure you can go over the code above and actually understand
// every step of what's going on.

// So much for `vec_min`. Let us now reconsider `print_number_or_nothing`. That function
// really belongs pretty close to the type `NumberOrNothing`. In C++ or Java, you would
// probably make it a method of the type. In Rust, we can achieve something very similar
// by providing an *inherent implementation*.
impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
}
// So, what just happened? Rust separates code from data, so the definition of the
// methods on an `enum` (and also on `struct`, which we will learn about later)
// is independent of the definition of the type. `self` is like `this` in other
// languages, and its type is always implicit. So `print` is now a method that
// takes as first argument a `NumberOrNothing`, just like `print_number_or_nothing`.
// 
// Try making `number_or_default` from above an inherent method as well!

// With our refactored functions and methods, `main` now looks as follows:
fn read_vec() -> Vec<i32> {
    vec![18,5,7,2,9,27]
}
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
// You will have to replace `part00` by `part01` in the `main` function in
// `main.rs` to run this code.

// **Exercise 01.1**: Write a funtion `vec_avg` that computes the average value of a `Vec<i32>`.
// 
// *Hint*: `vec.len()` returns the length of a vector `vec`.

// [index](main.html) | [previous](part00.html) | [next](part02.html)
