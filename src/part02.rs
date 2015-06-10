// Rust-101, Part 02: Generic types, Traits
// ========================================

use std;

// Let us for a moment reconsider the type `NumberOrNothing`. Isn't it a bit
// annoying that we had to hard-code the type `i32` in there? What if tomorrow,
// we want a `CharOrNothing`, and later a `FloatOrNothing`? Certainly we don't
// want to re-write the type and all its inherent methods.

// The solution to this is called *generics* or *polymorphism* (the latter is Greek,
// meaning "many shapes"). You may know something similar from C++ (where it's called
// *templates*) or Java, or one of the many functional languages. So here, we define
// a generic type `SomethingOrNothing`.
enum SomethingOrNothing<T>  {
    Something(T),
    Nothing,
}
use self::SomethingOrNothing::{Something,Nothing};
// What this does is to define an entire family of types: We can now write
// `SomethingOrNothing<i32>` to get back our `NumberOrNothing`, but we
// can also write `SomethingOrNothing<bool>` or even `SomethingOrNothing<SomethingOrNothing<i32>>`.
// In fact, such a type is so useful that it is already present in the standard
// library: It's called an *option type*, written `Option<T>`.
// Go check out its [documentation](http://doc.rust-lang.org/stable/std/option/index.html)!
// (And don't worry, there's indeed lots of material mentioned there that we did not cover yet.)

// **Exercise**: Write functions converting between `SomethingOrNothing<T>` and `Option<T>`. You will have to use
// the names of the constructor of `Option`, which you can find in the documentation I linked above.
// 
// Here's a skeleton for your solution, you only have to fill in the function bodies.
// (`panic!` is, again, a macro - this one terminates execution when it is reached).
// 
// Notice the syntax for giving generic implementations to generic types: Think of the first `<T>` 
// as *declaring* a type variable ("I am doing something for all types `T`"), and the second `<T>` as
// *using* that variable ("The thing I do, is implement `SomethingOrNothing<T>`").
impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        panic!("Not yet implemented.")
    }

    fn to_option(self) -> Option<T> {
        panic!("Not yet implemented.")
    }
}
// Inside an `impl`, `Self` refers to the type we are implementing things for. Here, it is
// an alias for `SomethingOrNothing<T>`.
// Remember that `self` is the `this` of Rust, and implicitly has type `Self`.
// 
// Observe how `new` does *not* have a `self` parameter. This corresponds to a `static` method
// in Java or C++. In fact, `new` is the Rust convention for defining constructors: They are
// nothing special, just static functions returning `Self`.

// You can call static functions, and in particular constructors, as follows:
fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

// Now that we have a generic `SomethingOrNothing`, wouldn't it be nice to also gave a generic
// `vec_min`? Of course, we can't take the minimum of a vector of *any* type. It has to be a type
// supporting a `min` operation. Rust calls such properties that we may demand of types *traits*.

// So, as a first step towards a generic `vec_min`, we define a `Minimum` trait.
// For now, just ignore the `Copy`, we will come back to this point later.
// A `trait` is a lot like interfaces in Java: You define a bunch of functions
// you want to have implemented, and their argument and return types.
trait Minimum : Copy {
    fn min(a: Self, b: Self) -> Self;
}

// Now we can write `vec_min`, generic over a type `T` that we demand to satisfy the `Minimum` trait.
// This is called a *trait bound*.
// The only difference to the version from the previous part is that we call `T::min` (the `min`
// function provided for type `T`) instead of `std::cmp::min`.
// 
// Notice a crucial difference to templates in C++: We actually have to declare which traits
// we want the type to satisfy. If we left away the `Minimum`, Rust would have complained that
// we cannot call `min`. Just try it! There is no reason to believe that `T` provides such an operation.
// This is in strong contrast to C++, where the compiler only checks such details when the
// function is actually used.
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

// To make the function usable with a `Vec<i32>`, we implement the `Minimum` trait for `i32`.
impl Minimum for i32 {
    fn min(a: Self, b: Self) -> Self {
        std::cmp::min(a, b)
    }
}

// In order to run our code and see the result, we again provide a `print` function.
// This also shows that we can have multiple `impl` blocks for the same type, and we
// can provide some methods only for certain instances of a generic type.
impl SomethingOrNothing<i32> {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

// Now we are again ready to run our code. Remember to change `main.rs` appropriately.
// Rust figures out automatically that we want the `T` of `vec_min` to be `i32`, and
// that `i32` implements `Minimum` and hence all is good.
// 
// In case you are worried about performance, note that Rust performs *monomorphisation*
// of generic functions: When you call `vec_min` with `T` being `i32`, Rust essentially goes
// ahead and creates a copy of the function for this particular type, filling in all the blanks.
// In this case, the call to `T::min` will become a call to our implementation *statically*. There is
// no dynamic dispatch, like there would be for Java interface methods or C++ `virtual` methods.
// This behavior is similar to C++ templates. The optimizer (Rust is using LLVM) then has all the
// information it could want to, e.g., inline function calls.
fn read_vec() -> Vec<i32> {
    vec![18,5,7,3,9,27]
}
pub fn part_main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}

// If this printed `3`, then you generic `vec_min` is working!
// 
// Before going on, take a moment to ponder the flexibility of Rust's take on abstraction:
// We just defined our own, custom trait (interface), and then implemented that trait
// *for an existing type*. With the hierarchical approach of, e.g., C++ or Java,
// that's not possible: We cannot make an existing type suddenly also inherit from our abstract base class.

// **Exercise**: Define a trait `Print` to write a generic version of `SomethingOrNothing::print`.
// Implement that trait for `i32`, and change the code above to use it.
// I will again provide a skeleton for this solution. It also shows how to attach bounds to generic
// implementations (just compare it to the `impl` block from the previous exercise).
// You can read this as "For all types `T` satisfying the `Print` trait, I provide an implementation
// for `SomethingOrNothing<T>`".
// 
// Notice that I called the function on `SomethingOrNothing` `print2` to disambiguate from the `print` defined above.
// 
// *Hint*: There is a macro `print!` for printing without appending a newline.
trait Print {
    /* Add things here */
}
impl<T: Print> SomethingOrNothing<T> {
    fn print2(self) {
        panic!("Not yet implemented.")
    }
}

// [index](main.html) | [previous](part01.html) | [next](part03.html)
