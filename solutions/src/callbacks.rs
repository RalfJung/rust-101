use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Callbacks {
    callbacks: Vec<Rc<RefCell<FnMut(i32)>>>,
}

impl Callbacks {
    pub fn new() -> Self {
        Callbacks { callbacks: Vec::new() }                      /*@*/
    }

    pub fn register<F: FnMut(i32)+'static>(&mut self, callback: F) {
        let cell = Rc::new(RefCell::new(callback));
        self.callbacks.push(cell);                                  /*@*/
    }

    pub fn call(&mut self, val: i32) {
        for callback in self.callbacks.iter() {
            // We have to *explicitly* borrow the contents of a `RefCell`.
            //@ At run-time, the cell will keep track of the number of outstanding shared and mutable borrows,
            //@ and panic if the rules are violated. Since this function is the only one that borrow the
            //@ environments of the closures, and this function requires a *mutable* borrow of `self`, we know this cannot
            //@ happen. <br />
            //@ For this check to be performed, `closure` is a *guard*: Rather than a normal borrow, `borrow_mut` returns
            //@ a smart pointer (`RefMut`, in this case) that waits until is goes out of scope, and then
            //@ appropriately updates the number of active borrows.
            //@ 
            //@ The function would still typecheck with an immutable borrow of `self` (since we are
            //@ relying on the interior mutability of `self`), but then it could happen that a callback
            //@ will in turn trigger another round of callbacks, so that `call` would indirectly call itself.
            //@ This is called reentrancy. It would imply that we borrow the closure a second time, and
            //@ panic at run-time. I hope this also makes it clear that there's absolutely no hope of Rust
            //@ performing these checks statically, at compile-time: It would have to detect reentrancy!
            let mut closure = callback.borrow_mut();
            // Unfortunately, Rust's auto-dereference of pointers is not clever enough here. We thus have to explicitly
            // dereference the smart pointer and obtain a mutable borrow of the target.
            (&mut *closure)(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::*;

    #[test]
    #[should_panic]
    fn test_reentrant() {
        let c = Rc::new(RefCell::new(Callbacks::new()));
        c.borrow_mut().register(|val| println!("Callback called: {}", val) );

        {
            let c2 = c.clone();
            c.borrow_mut().register(move |val| {
                let mut guard = c2.borrow_mut();
                println!("Callback called with {}, ready to go for nested call.", val);
                guard.call(val+val)
            } );
        }

        // We do a clone, and call `call` on that one. This makes sure that it's not our `RefCell` that complains about two mutable borrows,
        // but rather the `RefCell` inside the `CallbacksMut`.
        let mut c2: Callbacks = c.borrow().clone();
        drop(c);
        c2.call(42);
    }
}