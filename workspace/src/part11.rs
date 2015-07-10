// Rust-101, Part 11: Trait Objects, Box (WIP)
// ===========================================


mod callbacks {
    // For now, we just decide that the callbakcs have an argument of type `i32`.
    struct CallbacksV1<F: FnMut(i32)> {
        callbacks: Vec<F>,
    }

    /* struct CallbacksV2 {
        callbacks: Vec<FnMut(i32)>,
    } */

    struct Callbacks {
        callbacks: Vec<Box<FnMut(i32)>>,
    }

    impl Callbacks {
        // Now we can provide some functions. The constructor should be straight-forward.
        fn new() -> Self {
            unimplemented!()
        }

        // Registration simply stores the callback.
        fn register(&mut self, callback: Box<FnMut(i32)>) {
            unimplemented!()
        }

        // And here we call all the stored callbacks.
        fn call(&mut self, val: i32) {
            // Since they are of type `FnMut`, we need to mutably iterate. Notice that boxes dereference implicitly.
            for callback in self.callbacks.iter_mut() {
                unimplemented!()
            }
        }
    }

    // Now we are read for the demo.
    pub fn demo() {
        let mut c = Callbacks::new();
        c.register(Box::new(|val| println!("Callback 1: {}", val)));

        c.call(0);

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

    use std::rc;

    #[derive(Clone)]
    struct Callbacks {
        callbacks: Vec<rc::Rc<Fn(i32)>>,
    }

    // The methods on these clonable callbacks are just like the ones above.
    impl Callbacks {
        fn new() -> Self {
            unimplemented!()
        }

        fn register(&mut self, callback: rc::Rc<Fn(i32)>) {
            unimplemented!()
        }

        fn call(&mut self, val: i32) {
            // We only need a shared iterator here. `Rc` also implicitly dereferences, so we can just call the callback.
            for callback in self.callbacks.iter() {
                unimplemented!()
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

