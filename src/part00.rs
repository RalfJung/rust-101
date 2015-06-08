// [index](main.html) | previous | [next](part01.html)

use std;

// Rust-101, Part 00: Algebraic datatypes, expressions
// ===================================================

// As a starter, we want to write a function that computes the minimum of a list.
// First, we need to write down the signature of the function: The types of its arguments and
// of the return value. In the case of our minimum function,
// we may be inclined to say that it returns a number. But then we would be in trouble: What's
// the minimum of an empty list? The type of the function says we have to return *something*.
// We could just choose 0, but that would be kind of arbitrary. What we need
// is a type that is "a number, or nothing". Such a type (of multiple exclusive options)
// is called an "algebraic datatype", and Rust lets us define such types with the keyword `enum`.
// Coming from C(++), you can think of such a type as a `union`, together with a field that
// stores the variant of the union that's currently used.

enum NumberOrNothing {
    Number(i32),
    Nothing
}

// Notice that `i32` is the type of (signed, 32-bit) integers. To write down the type of
// the minimum function, we need just one more ingredient: `Vec<i32>` is the type of
// (growable) arrays of numbers, and we will use that as our list type.
// Observe how in Rust, the function type comes *after* the arguments.

fn vec_min_try1(vec: Vec<i32>) -> NumberOrNothing {
    // First, we need some variable to store the minimum as computed so far.
    // Since we start out with nothing computed, this will again be a 
    // "number or nothing". Notice that we do not have to write a type
    // next to `min`, Rust can figure that out automatically (a bit like
    // `auto` in C++11). Also notice the `mut`: In Rust, variables are
    // immutable per default, and you need to tell Rust if you want
    // to change a variable later.
    let mut min = NumberOrNothing::Nothing;

    // Now we want to *iterate* over the list. Rust has some nice syntax for
    // iterators:
    for el in vec {
        // So `el` is al element of the list. We need to update `min` accordingly, but how do we get the current
        // number in there? This is what pattern matching can do:
        match min {
            NumberOrNothing::Nothing => {
                // `min` is currently nothing, so let's just make it the number `el`.
                min = NumberOrNothing::Number(el);
            },
            NumberOrNothing::Number(n) => {
                // `min` is currently the number `n`, so let's compute the new minimum and store it.
                let new_min = std::cmp::min(n, el);
                min = NumberOrNothing::Number(new_min);
            }
        }
    }
    // Finally, we return the result of the computation.
    return min;
}

// Phew. We wrote our first Rust function! But all this `NumberOrNothing::` is getting kind of
// ugly. Can't we do that nicer? Indeed, we can: The following line tells Rust to take
// the constructors of `NumberOrNothing` into the local namespace:
use self::NumberOrNothing::{Number,Nothing};
// Try moving that above the function, and removing all the occurrences `NumberOrNothing::`.
// Things should still compile, now being much less verbose!

// However, the code is still not "idiomatic Rust code". To understand why, it is important to
// understand that Rust is an "expression-based" language. This means that most of the
// terms you write down are not just *statements* (executing code), but *expressions*
// (returning a value). This applies even to the body of entire functions!

// For example, consider `sqr`. Between the curly braces, we are giving the *expression*
// that computes the return value. So we can just write `i * i`, the expression that
// returns the square if `i`, and make that our return value! Note that this is
// very close to how mathematicians write down functions (but with more types).
fn sqr(i: i32) -> i32 { i * i }

// Conditionals are also just expressions. You can compare this to the ternary `? :` operator
// from languages like C.
fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

// And the same applies to case distinction with `match`: Every `arm` of the match
// gives the expression that is returned in the respective case.
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
        // First of all, notice that all we do here is compute a new value for `min`, and that we
        // will always end up calling the `Number` constructor. In Rust, the structure of the code
        // can express this uniformity as follows:
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

// To call this function, we now just need a list! Of course, ultimately we want to ask the user for
// a list of numbers, but for now, let's just hard-code something:

fn read_vec() -> Vec<i32> {
    // `vec!` is a *macro* (as you can tell from the `!`) that constructs a constant `Vec` with the given
    // elements.
    vec![18,5,7,1,9,27]
}

// Finally, let's call our functions and run the code!
// But, wait, we would like to actually see something. Of course Rust can print numbers,
// but after calling `vec_min`, we have a `NumberOrNothing`. So let's write a small helper
// function that can prints such values.

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("The number is: <nothing>"),
        Number(n) => println!("The number is: {}", n),
    };
}

// So putting it all together - if you type `cargo run`, it will
// run the following code:

pub fn part_main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}

// Yay, it said "1"! That's actually the right answer. Okay, we could have
// computed that ourselves, but that's besides the point. More importantly:
// You completed the first part of the course.

// [index](main.html) | previous | [next](part01.html)
