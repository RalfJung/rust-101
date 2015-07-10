// Rust-101, Part 11: Trait Objects, Box (WIP)
// ===========================================

//@ Now that we know about closures, let's have some fun with them. We will try to implement some kind of generic "callback"
//@ mechanism, providing two functions: Registering a new callback, and calling all registered callbacks. There will be two
//@ versions, so to avoid clashes of names, we put them into modules.

mod callbacks {
    //@ First of all, we need to find a way to store the callbacks. Clearly, there will be a `Vec` involves, so that we can
    //@ always grow the number of registered callbacks. A callback will be a closure, i.e., something implementing
    //@ `FnMut(i32)` (we want to call this multiple times, so clearly `FnOnce` would be no good). So our first attempt may be the following.
    // For now, we just decide that the callbakcs have an argument of type `i32`.
    struct CallbacksV1<F: FnMut(i32)> {
        callbacks: Vec<F>,
    }
    //@ However, this will not work. Remember how the "type" of a closure is specific to the environment of captures variables. Different closures
    //@ all implementing `FnMut(i32)` may be different types. However, a `Vec<F>` is a *uniformly typed* vector.

    //@ We will this need a way to store things of *different* types in the same vector. We know all these types implement `FnMut(i32)`. For this scenario,
    //@ Rust provides *trait objects*: The truth is, that `FnMut(i32)` is not just a trait. It is also a type, that can be given to anything implementing
    //@ this trait. So, we may write:
    /* struct CallbacksV2 {
        callbacks: Vec<FnMut(i32)>,
    } */
    //@ But, Rust complains about this definition. It says something about "Sized". What's the trouble? See, for many things we want to do, it is crucial that
    //@ Rust knows the precise, fixed size of the type - that is, how large will this type be when represented in memory. For example, for a `Vec`, the
    //@ elements are stored one right after the other. How should that be possible, without a fixed size? The trouble is, `FnMut(i32)` could be of any size.
    //@ We don't know how large that "type that implemenets `FnMut(i32)`" is. Rust calls this an *unsized* type. Whenever we introduce a type variable, Rust
    //@ will implicitly add a bound to that variable, demanding that it is sized. That's why we did not have to worry about this so far.
    //@ You can, btw, opt-out of this implicit bound by saying `T: ?Sized`. Then `T` may or may not be sized.

    //@ So, what can we do, if we can't store the callbacks in a vector? We can put them in a box. Semantically, `Box<T>` is a lot like `T`: You fully own
    //@ the data stored there. On the machine, however, `Box<T>` is a *pointer* to `T`. It is a lot like `std::unique_ptr` in C++. In our current example,
    //@ the important bit is that since it's a pointer, `T` can be unsized, but `Box<T>` itself will always be sized. So we can put it in a `Vec`.
    struct Callbacks {
        callbacks: Vec<Box<FnMut(i32)>>,
    }

    impl Callbacks {
        // Now we can provide some functions. The constructor should be straight-forward.
        fn new() -> Self {
            Callbacks { callbacks: Vec::new() }                     /*@*/
        }

        // Registration simply stores the callback.
        fn register(&mut self, callback: Box<FnMut(i32)>) {
            self.callbacks.push(callback);                          /*@*/
        }

        // And here we call all the stored callbacks.
        fn call(&mut self, val: i32) {
            // Since they are of type `FnMut`, we need to mutably iterate. Notice that boxes dereference implicitly.
            for callback in self.callbacks.iter_mut() {
                callback(val);                                      /*@*/
            }
        }
    }

    // Now we are read for the demo.
    pub fn demo() {
        let mut c = Callbacks::new();
        c.register(Box::new(|val| println!("Callback 1: {}", val)));

        c.call(0);

        //@ We can even register callbacks that modify their environment. Rust will again attempt to borrow `count`. However,
        //@ that doesn't work out this time: Since we want to put this thing in a `Box`, it could live longer than the function
        //@ we are in. Then the borrow of `count` would become invalid. However, we can tell rust to `move` ownership of the
        //@ variable into the closure. Its environment will then contain an `usize` rather than a `&mut uszie`, and have
        //@ no effect on this local variable anymore.
        let mut count: usize = 0;
        c.register(Box::new(move |val| { count = count+1; println!("Callback 2, {}. time: {}", count, val); } ));
        c.call(1);
        c.call(2);
    }

}

// Remember to edit `main.rs` to run the demo.
pub fn main() {
    callbacks::demo();
}

mod callbacks_clone {
    //@ So, this worked great, didn't it! There's one point though that I'd like to emphasize: One cannot `clone` a closure.
    //@ Hence it becomes impossibly to implement `Clone` for our `Callbacks` type. What could we do about this?

    //@ You already learned about `Box` above. `Box` is historically a very special type in Rust (though it lost most of its
    //@ particularities by now, and people are working on making it just a normal library type). Effectively, however, it is
    //@ just an example of a *smart pointer*: It's like a pointer (i.e., a borrow), but with some additional smarts to it. For
    //@ `Box`, that's the part about ownership. Once you drop the box, the content it points to will also be deleted.
    //@ 
    //@ Another example of a smart pointer in Rust is `Rc<T>`. This is short for *reference-counter*, so you can already guess how
    //@ this pointer is smart: It has a reference count. You can `clone` an `Rc` as often as you want, that doesn't affect the
    //@ data it contains at all. It only creates more references to the same data. Once all the references are gone, the data is
    //@ deleted.
    //@ 
    //@ Wait a moment, you may here. Multiple references to the same data? That's aliasing! Indeed, we have to be careful here.
    //@ Once data is stored in an `Rc`, is is read-only: By dereferencing the smart `Rc`, you can only get a shared borrow of the data.
    use std::rc;

    //@ Because of this read-only restriction, we cannot use `FnMut` here: We'd be unable to call the function with a mutable borrow
    //@ of it's environment! So we have to go with `Fn`. We wrap that in an `Rc`, and then Rust happily derives `Clone` for us.
    #[derive(Clone)]
    struct Callbacks {
        callbacks: Vec<rc::Rc<Fn(i32)>>,
    }

    // The methods on these clonable callbacks are just like the ones above.
    impl Callbacks {
        fn new() -> Self {
            Callbacks { callbacks: Vec::new() }                     /*@*/
        }

        fn register(&mut self, callback: rc::Rc<Fn(i32)>) {
            self.callbacks.push(callback);                          /*@*/
        }

        fn call(&mut self, val: i32) {
            // We only need a shared iterator here. `Rc` also implicitly dereferences, so we can just call the callback.
            for callback in self.callbacks.iter() {
                callback(val);                                      /*@*/
            }
        }
    }

    // The demo works just as above. Our counting callback doesn't work anymore though, because we are using `Fn` now.
    fn demo() {
        let mut c = Callbacks::new();
        c.register(rc::Rc::new(|val| println!("Callback 1: {}", val)));

        c.call(0);
        c.call(1);
    }
}

// **Exercise 11.1**: We made the arbitrary choice of using `i32` for the arguments. Generalize the data-structures above
// to work with an arbitrary type `T` that's passed to the callbacks. Since you need to call multiple callbacks with the
// same `t: T`, you will either have to restrict `T` to `Copy` types, or pass a borrow.

//@ [index](main.html) | [previous](part10.html) | [next](main.html)
