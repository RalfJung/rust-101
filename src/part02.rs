// Rust-101, Part 02: Generic types (WIP)
// ================================

use std;

// Let us for a moment reconsider the type `NumberOrNothing`. Isn't it a bit
// annoying that we had to hard-code the type `i32` in there? What if tomorrow,
// we want a `CharOrNothing`, and later a `FloatOrNothing`? Certainly we don't
// want to re-write the type and all its inherent methods.
// 
// The solution to this is called *generics* or *polymorphism* (the latter is Greek,
// meaning "many shapes"). You may know something similar from C++ (where it's called
// *templates*) or Java, or one of the many functional languages. A generic
// `SomethingOrNothing` type looks as follows:
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

// [index](main.html) | [previous](part01.html) | [next](part03.html)
