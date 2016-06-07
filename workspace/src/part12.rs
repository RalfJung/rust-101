// Rust-101, Part 12: Rc, Interior Mutability, Cell, RefCell
// =========================================================

use std::rc::Rc;
use std::cell::{Cell, RefCell};



#[derive(Clone)]
struct Callbacks {
    callbacks: Vec<Rc<Fn(i32)>>,
}

impl Callbacks {
    pub fn new() -> Self {
        Callbacks { callbacks: Vec::new() }
    }

    // Registration works just like last time, except that we are creating an `Rc` now.
    pub fn register<F: Fn(i32)+'static>(&mut self, callback: F) {
        unimplemented!()
    }

    pub fn call(&self, val: i32) {
        // We only need a shared iterator here. Since `Rc` is a smart pointer, we can directly call the callback.
        for callback in self.callbacks.iter() {
            unimplemented!()
        }
    }
}

// Time for a demo!
fn demo(c: &mut Callbacks) {
    c.register(|val| println!("Callback 1: {}", val));
    c.call(0); c.clone().call(1);
}

pub fn main() {
    let mut c = Callbacks::new();
    demo(&mut c);
}

// ## Interior Mutability

// So, let us put our counter in a `Cell`, and replicate the example from the previous part.
fn demo_cell(c: &mut Callbacks) {
    {
        let count = Cell::new(0);
        // Again, we have to move ownership of the `count` into the environment closure.
        c.register(move |val| {
            // In here, all we have is a shared reference of our environment. But that's good enough for the `get` and `set` of the cell!
            let new_count = count.get()+1;
            count.set(new_count);
            println!("Callback 2: {} ({}. time)", val, new_count);
        } );
    }

    c.call(2); c.clone().call(3);
}


// ## `RefCell`

// Our final version of `Callbacks` puts the closure environment into a `RefCell`.
#[derive(Clone)]
struct CallbacksMut {
    callbacks: Vec<Rc<RefCell<FnMut(i32)>>>,
}

impl CallbacksMut {
    pub fn new() -> Self {
        CallbacksMut { callbacks: Vec::new() }
    }

    pub fn register<F: FnMut(i32)+'static>(&mut self, callback: F) {
        unimplemented!()
    }

    pub fn call(&mut self, val: i32) {
        for callback in self.callbacks.iter() {
            // We have to *explicitly* borrow the contents of a `RefCell` by calling `borrow` or `borrow_mut`.
            let mut closure = callback.borrow_mut();
            // Unfortunately, Rust's auto-dereference of pointers is not clever enough here. We thus have to explicitly
            // dereference the smart pointer and obtain a mutable reference to the content.
            (&mut *closure)(val);
        }
    }
}

// Now we can repeat the demo from the previous part - but this time, our `CallbacksMut` type
// can be cloned.
fn demo_mut(c: &mut CallbacksMut) {
    c.register(|val| println!("Callback 1: {}", val));
    c.call(0);

    {
        let mut count: usize = 0;
        c.register(move |val| {
            count = count+1;
            println!("Callback 2: {} ({}. time)", val, count);
        } );
    }
    c.call(1); c.clone().call(2);
}

// **Exercise 12.1**: Write some piece of code using only the available, public interface of `CallbacksMut` such that a reentrant call to a closure
// is happening, and the program panics because the `RefCell` refuses to hand out a second mutable borrow of the closure's environment.

