// Rust-101, Part 16: Unsafe Rust, Drop
// ====================================

use std::ptr;
use std::mem;
use std::marker::PhantomData;


// A node of the list consists of the data, and two node pointers for the predecessor and successor.
struct Node<T> {
    next: NodePtr<T>,
    prev: NodePtr<T>,
    data: T,
}
// A node pointer is a *mutable raw pointer* to a node.
type NodePtr<T> = *mut Node<T>;

// The linked list itself stores pointers to the first and the last node. In addition, we tell Rust that this type
// will own data of type `T`.
pub struct LinkedList<T> {
    first: NodePtr<T>,
    last:  NodePtr<T>,
    _marker: PhantomData<T>,
}


unsafe fn raw_into_box<T>(r: *mut T) -> Box<T> {
    mem::transmute(r)
}
fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe { mem::transmute(b) }
}

impl<T> LinkedList<T> {
    // A new linked list just contains null pointers. `PhantomData` is how we construct any `PhantomData<T>`.
    pub fn new() -> Self {
        LinkedList { first: ptr::null_mut(), last: ptr::null_mut(), _marker: PhantomData }
    }

    // This function adds a new node to the end of the list.
    pub fn push_back(&mut self, t: T) {
        // Create the new node, and make it a raw pointer.
        let new = Box::new( Node { data: t, next: ptr::null_mut(), prev: self.last } );
        let new = box_into_raw(new);
        // Update other pointers to this node.
        if self.last.is_null() {
            debug_assert!(self.first.is_null());
            // The list is currently empty, so we have to update the head pointer.
            unimplemented!()
        } else {
            debug_assert!(!self.first.is_null());
            // We have to update the `next` pointer of the tail node.
            unimplemented!()
        }
        // Make this the last node.
        self.last = new;
    }

    // **Exercise 16.1**: Add some more operations to `LinkedList`: `pop_back`, `push_front` and `pop_front`.
    // Add testcases for `push_back` and all of your functions. The `pop` functions should take `&mut self`
    // and return `Option<T>`.

    // Next, we are going to provide an iterator.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.first, _marker: PhantomData  }
    }
}


pub struct IterMut<'a, T> where T: 'a {
    next: NodePtr<T>,
    _marker: PhantomData<&'a mut LinkedList<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // The actual iteration is straight-forward: Once we reached a null pointer, we are done.
        if self.next.is_null() {
            None
        } else {
            // Otherwise, we can convert the next pointer to a reference, get a reference to the data
            // and update the iterator.
            let next = unsafe { &mut *self.next };
            let ret = &mut next.data;
            unimplemented!()
        }
    }
}


// **Exercise 16.2**: Add a method `iter` and a type `Iter` providing iteration for shared references.
// Add testcases for both kinds of iterators.

// ## `Drop`

impl<T> Drop for LinkedList<T> {
    // The destructor itself is a method which takes `self` in mutably borrowed form. It cannot own `self`, because then
    // the destructor of `self` would be called at the end of the function, resulting in endless recursion.
    fn drop(&mut self) {
        let mut cur_ptr = self.first;
        while !cur_ptr.is_null() {
            // In the destructor, we just iterate over the entire list, successively obtaining ownership
            // (`Box`) of every node. When the box is dropped, it will call the destructor on `data` if
            // necessary, and subsequently free the node on the heap.
            let cur = unsafe { raw_into_box(cur_ptr) };
            cur_ptr = cur.next;
            drop(cur);
        }
    }
}

// ## The End

