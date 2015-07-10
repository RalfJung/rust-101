// Rust-101, Part 11: Trait Objects, Box, Rc, Lifetime bounds
// ==========================================================

mod callbacks {
    // For now, we just decide that the callbacks have an argument of type `i32`.
    struct CallbacksV1<F: FnMut(i32)> {
        callbacks: Vec<F>,
    }

    /* struct CallbacksV2 {
        callbacks: Vec<FnMut(i32)>,
    } */

    pub struct Callbacks {
        callbacks: Vec<Box<FnMut(i32)>>,
    }

    impl Callbacks {
        // Now we can provide some functions. The constructor should be straight-forward.
        pub fn new() -> Self {
            unimplemented!()
        }

        // Registration simply stores the callback.
        pub fn register(&mut self, callback: Box<FnMut(i32)>) {
            unimplemented!()
        }

        // And here we call all the stored callbacks.
        pub fn call(&mut self, val: i32) {
            // Since they are of type `FnMut`, we need to mutably iterate. Notice that boxes dereference implicitly.
            for callback in self.callbacks.iter_mut() {
                unimplemented!()
            }
        }
    }

    // Now we are ready for the demo.
    pub fn demo(c: &mut Callbacks) {
        c.register(Box::new(|val| println!("Callback 1: {}", val)));
        c.call(0);

        let mut count: usize = 0;
        c.register(Box::new(move |val| { count = count+1; println!("Callback 2, {}. time: {}", count, val); } ));
        c.call(1); c.call(2);
    }
}

// Remember to edit `main.rs` to run the demo.
pub fn main() {
    let mut c = callbacks::Callbacks::new();
    callbacks::demo(&mut c);
}

mod callbacks_clone {

    use std::rc::Rc;

    #[derive(Clone)]
    pub struct Callbacks {
        callbacks: Vec<Rc<Fn(i32)>>,
    }

    impl Callbacks {
        pub fn new() -> Self {
            unimplemented!()
        }

        // For the `register` function, we don't actually have to use trait objects in the argument.
        
        pub fn register<F: Fn(i32)+'static>(&mut self, callback: F) {
            unimplemented!()
        }

        pub fn call(&mut self, val: i32) {
            // We only need a shared iterator here. `Rc` also implicitly dereferences, so we can simply call the callback.
            for callback in self.callbacks.iter() {
                unimplemented!()
            }
        }
    }

    // The demo works just as above. Our counting callback doesn't work anymore though, because we are using `Fn` now.
    fn demo(c: &mut Callbacks) {
        c.register(|val| println!("Callback 1: {}", val));
        c.call(0); c.call(1);
    }
}

// **Exercise 11.1**: We made the arbitrary choice of using `i32` for the arguments. Generalize the data-structures above
// to work with an arbitrary type `T` that's passed to the callbacks. Since you need to call multiple callbacks with the
// same `t: T`, you will either have to restrict `T` to `Copy` types, or pass a borrow.


