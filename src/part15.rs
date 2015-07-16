// Rust-101, Part 15: Interior Mutability (cont.), RefCell, Cell, Drop
// ===================================================================

//@ [`RefCell`](http://doc.rust-lang.org/beta/std/cell/struct.RefCell.html)
//@ [`is very much like `RwLock`, but it's not thread-safe: "Locking" is done without atomic operations.
//@ One can also see it as a dynamically checked version of Rust's usual borrowing rules. You have to explicitly say
//@ when you want to borrow the data in there shared, or mutably, and Rust will complain at run-time if you have
//@ a mutable borrow while any other borrow is active. You can then write programs that Rust may otherwise not
//@ accept. Sending a shared borrow to this to another thread is dangerous, as the checks are not performed in
//@ a thread-safe manner. However, sending the *entire* `RefCell` is okay, because there's only ever one owner, and all
//@ we need to ensure is that everybody attempting to borrow is in the same thread as the owner. <br/>
//@ [`Cell<T>`](http://doc.rust-lang.org/beta/std/cell/struct.Cell.html) is like a stripped-down version of `RefCell<T>`: It doesn't allow
//@ you to borrow its content. Instead, it has a methods `get` and `set` to change the value stored in the cell, and to copy it out.
//@ For obvious reasons, this requires `T` to be `Copy`.
//@ 
//@ You can also think about all these types coming from the other end: Starting with `Cell`, we have a primitive for
//@ interior mutability that provides `get` and `set`, both just requiring a shared borrow. Think of these functions as
//@ mutating the *content* of the cell, but not the cell itself, the container. (Just like in ML, where assignment to a 
//@ `ref` changes the content, not the location.) However, due to the ownership discipline, `Cell` only works for types
//@ that are `Copy`. Hence we also have `RefCell`, which allows working with the data right in the cell, rather than
//@ having to copy it out. `RefCell` uses non-atomic operations for this purpose, so for the multi-threaded setting, there's
//@ the thread-safe `RwLock`. And finally, in case a distinction between readers and writers is not helpful, one can use the
//@ more efficient `Mutex`.

//@ [index](main.html) | [previous](part14.html) | [next](main.html)
