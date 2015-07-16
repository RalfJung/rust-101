// Rust-101, Part 14: Mutex, Interior Mutability, Sync
// ===================================================

use std::sync::{Arc, Mutex};
use std::thread;

//@ We already saw that we can use `Arc` to share memory between threads. However, `Arc` can only provide *read-only*
//@ access to memory: Since there is aliasing, Rust cannot, in general, permit mutation. If however,
//@ some care would be taken at run-time, then mutation would still be all right: We have to ensure that whenever
//@ someone changes the data, nobody else is looking at it.  In other words, we need a *critical section* or (as it
//@ is called in Rust) a [`Mutex`](http://doc.rust-lang.org/stable/std/sync/struct.Mutex.html). Some other languages also call this a *lock*.
//@ 
//@ As an example, let us write a concurrent counter. As usual in Rust, we first have to think about our data layout.
//@ In case of the mutex, this means we have to declare the type of the data that we want to be protected. In Rust,
//@ a `Mutex` protects data, not code - and it is impossible to access the data in any other way. This is generally considered
//@ good style, but other languages typically lack the ability to actually enforce this.
//@ Of course, we want multiple threads to have access to this `Mutex`, so we wrap it in an `Arc`.
//@ 
//@ Rather than giving every field a name, a struct can also be defined by just giving a sequence of types (similar
//@ to how a variant of an `enum` is defined). This is called a *tuple struct*. It is often used when constructing
//@ a *newtype*, as we do here: `ConcurrentCounter` is essentially just a new name for `Arc<Mutex<usize>>`. However,
//@ is is a locally declared types, so we can give it an inherent implementation and implement traits for it. Since the
//@ field is private, nobody outside this module can even know the type we are wrapping.

// The derived `Clone` implementation will clone the `Arc`, so all clones will actually talk about the same counter.
#[derive(Clone)]
struct ConcurrentCounter(Arc<Mutex<usize>>);

impl ConcurrentCounter {
    // The constructor just wraps the constructors of `Arc` and `Mutex`.
    pub fn new(val: usize) -> Self {
        ConcurrentCounter(Arc::new(Mutex::new(val)))                /*@*/
    }

    //@ The core operation is, of course, `increment`. The type may be surprising at first: A shared borrow?
    //@ How can this be, since `increment` definitely modifies the counter? We already discussed above that `Mutex` is
    //@ a way to get around this restriction in Rust. This phenomenon of data that can be mutated through a shared
    //@ borrow is called *interior mutability*: We are changing the inner parts of the object, but seen from the outside,
    //@ this does not count as "mutation". This stands in contrast to *exterior mutability*, which is the kind of
    //@ mutability we saw so far, where one piece of data is replaced by something else of the same type. If you are familiar
    //@ with languages like ML, you can compare this to how something of type `ref` permits mutation, even though it is
    //@ itself a functional value (more precisely, a location) like all the others.
    //@ 
    //@ Interior mutability breaks the rules of Rust that I outlined earlier: There is aliasing (a shared borrow) and mutation.
    //@ The reason that this still works is careful programming of the primitives for interior mutability - in this case, that's
    //@ `Mutex`. It has to ensure with dynamic checks, at run-time, that things don't fall apart. In particular, it has to ensure
    //@ that the data covered by the mutex can only ever be accessed from inside a critical section. This is where Rust's type
    //@ system comes into play: With its discipline of ownership and borrowing, it can enforce such rules. Let's see how this goes.
    pub fn increment(&self, by: usize) {
        // `lock` on a mutex returns a *guard*, giving access to the data contained in the mutex.
        //@  (We will discuss the `unwrap` soon.) `.0` is how we access the first component of a tuple or a struct.
        let mut counter = self.0.lock().unwrap();
        //@ The guard is another example of a smart pointer, and it can be used as if it were a pointer to the data protected
        //@ by the lock.
        *counter = *counter + by;
        //@ At the end of the function, `counter` is dropped and the mutex is available again.
        //@ This can only happen when full ownership of the guard is given up. In particular, it is impossible for us
        //@ to borrow some of its content, release the lock of the mutex, and subsequently access the protected data without holding
        //@ the lock. Enforcing the locking discipline is expressible in the Rust type system, so we don't have to worry
        //@ about data races *even though* we are mutating shared memory!
        //@ 
        //@ One of the subtle aspects of locking is *poisoning*. If a thread panics while it holds a lock, it could leave the
        //@ data-structure in a bad state. The lock is hence considered *poisoned*. Future attempts to `lock` it will fail.
        //@ Above, we simply assert via `unwrap` that this will never happen. Alternatively, we could have a look at the poisoned
        //@ state and attempt to recover from it.
    }

    // The function `get` returns the current value of the counter.
    pub fn get(&self) -> usize {
        let counter = self.0.lock().unwrap();                       /*@*/
        *counter                                                    /*@*/
    }
}

// Now our counter is ready for action.
pub fn main() {
    let counter = ConcurrentCounter::new(0);

    // We clone the counter for the first thread, which increments it by 2 every 15ms.
    let counter1 = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep_ms(15);
            counter1.increment(2);
        }
    });

    // The second thread increments the counter by 3 every 20ms.
    let counter2 = counter.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep_ms(20);
            counter2.increment(3);
        }
    });

    // Now we watch the threads working on the counter.
    for _ in 0..50 {
        thread::sleep_ms(5);
        println!("Current value: {}", counter.get());
    }

    // Finally, we wait for all the threads to finish to be sure we can catch the counter's final value.
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("Final value: {}", counter.get());
}

// **Exercise 14.1**: Besides `Mutex`, there's also [`RwLock`](http://doc.rust-lang.org/stable/std/sync/struct.RwLock.html), which
// provides two ways of locking: One that grants only read-only access, to any number of concurrent readers, and another one
// for exclusive write access. (Notice that this is the same pattern we already saw with shared vs. mutable borrows.) Change
// the code above to use `RwLock`, such that multiple calls to `get` can be executed at the same time.
// 
// **Exercise 14.2**: Add an operation `compare_and_inc(&self, test: usize, by: usize)` that increments the counter by
// `by` *only if* the current value is `test`.

//@ ## Sync
//@ In part 12, we talked about types that are marked `Send` and thus can be moved to another thread. However, we did *not*
//@ talk about the question whether a borrow is `Send`. For `&mut T`, the answer is: It is `Send` whenever `T` is send.
//@ `&mut` allows moving values back and forth, it is even possible to [`swap`](http://doc.rust-lang.org/beta/std/mem/fn.swap.html)
//@ the contents of two mutably borrowed values. So in terms of concurrency, sending a mutable borrow is very much like
//@ sending full ownership, in the sense that it can be used to move the object to another thread.
//@ 
//@ But what about `&T`, a shared borrow? Without interior mutability, it would always be all-right to send such values.
//@ After all, no mutation can be performed, so there can be as many threads accessing the data as we like. In the
//@ presence of interior mutability though, the story gets more complicated. Rust introduces another marker trait for
//@ this purpose: `Sync`. A type `T` is `Sync` if `&T` is `Send`. Just like `Send`, `Sync` has a default implementation
//@ and is thus automatically implemented for a data-structure *if* all its members implement it.
//@ 
//@ Almost all the types we saw so far are `Sync`, with the exception of `Rc`. Remember that a shared borrow is good enough
//@ for cloning, and we don't want other threads to clone our local `Rc`, so it must not be `Sync`. The rule of `Mutex`
//@ is to enforce synchronization, so it should not be entirely surprising that `Mutex<T>` is `Send` *and* `Sync` provided that
//@ `T` is `Send`.
//@ 
//@ In the next part, we will learn about a type called `RefCell` that is `Send`, but not `Sync`.
//@ 
//@ You may be curious whether there is a type that's `Sync`, but not `Send`. There are indeed rather esoteric examples
//@ of such types, but that's not a topic I want to go into. In case you are curious, there's a
//@ [Rust RFC](https://github.com/rust-lang/rfcs/blob/master/text/0458-send-improvements.md), which contains a type `RcMut` that would be `Sync` and not `Send`.
//@ You may also be interested in [this blog post](https://huonw.github.io/blog/2015/02/some-notes-on-send-and-sync/) on the topic.

//@ [index](main.html) | [previous](part13.html) | [next](main.html)
