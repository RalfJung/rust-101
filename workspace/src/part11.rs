// Rust-101, Part 11: Trait Objects, Box, Lifetime bounds
// ======================================================


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
        self.callbacks.push(callback);
    }

    // We can also write a generic version of `register`, such that it will be instantiated with some concrete closure type `F`
    // and do the creation of the `Box` and the conversion from `F` to `FnMut(i32)` itself.
    
    pub fn register_generic<F: FnMut(i32)+'static>(&mut self, callback: F) {
        unimplemented!()
    }

    // And here we call all the stored callbacks.
    pub fn call(&mut self, val: i32) {
        // Since they are of type `FnMut`, we need to mutably iterate.
        for callback in self.callbacks.iter_mut() {
            unimplemented!()
        }
    }
}

// Now we are ready for the demo. Remember to edit `main.rs` to run it.
pub fn main() {
    let mut c = Callbacks::new();
    c.register(Box::new(|val| println!("Callback 1: {}", val)));
    c.call(0);

    {
        let mut count: usize = 0;
        c.register_generic(move |val| {
            count = count+1;
            println!("Callback 2: {} ({}. time)", val, count);
        } );
    }
    c.call(1); c.call(2);
}


// **Exercise 11.1**: We made the arbitrary choice of using `i32` for the arguments. Generalize the data structures above
// to work with an arbitrary type `T` that's passed to the callbacks. Since you need to call multiple callbacks with the
// same `t: T`, you will either have to restrict `T` to `Copy` types, or pass a reference.

