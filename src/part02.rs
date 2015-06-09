// Rust-101, Part 02: Generic types (WIP)
// ================================

use std;

// Let us for a moment reconsider the type `NumberOrNothing`. Isn't it a bit
// annoying that we had to hard-code the type `i32` in there? What if tomorrow,
// we want a `CharOrNothing`, and later a `FloatOrNothing`? Certainly we don't
// want to re-write the type and all its inherent methods.

// The solution to this is called *generics* or *polymorphism* (the latter is Greek,
// meaning "many shapes"). You may know something similar from C++ (where it's called
// *templates*) or Java, or one of the many functional languages. So here, we define
// a generic `SomethingOrNothing` type.
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

// Here's a skeleton for your solution, you only have to fill in the function bodies.
// (`panic!` is, again, a macro - this one terminates execution when it is reached).
// 
// Notice the syntax for giving generic implementations to generic types: Think of the first `<T>` 
// as *declaring* a type variable ("I am doing something for all types `T`"), and the second `<T>` as
// *using* that variable ("The thing I do, is implement `SomethingOrNothing<T>`").
impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        panic!("Not yet implemented.");
    }

    fn to_option(self) -> Option<T> {
        panic!("Not yet implemented.");
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
    

// [index](main.html) | [previous](part01.html) | next
