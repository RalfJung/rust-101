use std::ptr;
use std::mem;
use std::marker::PhantomData;

fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe { mem::transmute(b) }
}
unsafe fn raw_into_box<T>(r: *mut T) -> Box<T> {
    mem::transmute(r)
}

struct Node<T> {
    data: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}
type NodePtr<T> = *mut Node<T>;

pub struct LinkedList<T> {
    first: NodePtr<T>,
    last:  NodePtr<T>,
    _marker: PhantomData<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { first: ptr::null_mut(), last: ptr::null_mut(), _marker: PhantomData }
    }

    pub fn push_back(&mut self, t: T) {
        // Create the new node.
        let new = Box::new( Node { data: t, next: ptr::null_mut(), prev: self.last } );
        let new = box_into_raw(new);
        // Update other points to this node.
        if self.last.is_null() {
            debug_assert!(self.first.is_null());
            self.first = new;
        } else {
            debug_assert!(!self.first.is_null());
            unsafe { (*self.last).next  = new; }
        }
        // Make this the last node.
        self.last = new;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.last.is_null() {
            None
        } else {
            let last = self.last;
            let new_last = unsafe { (*self.last).prev };
            self.last = new_last;
            if new_last.is_null() {
                // The list is now empty.
                self.first = new_last;
            } else {
                unsafe { (*new_last).next = ptr::null_mut() };
            }
            let last = unsafe { raw_into_box(last) } ;
            Some(last.data)
        }
    }

    pub fn push_front(&mut self, t: T) {
        // Create the new node.
        let new = Box::new( Node { data: t, next: self.first, prev: ptr::null_mut() } );
        let new = box_into_raw(new);
        // Update other points to this node.
        if self.first.is_null() {
            debug_assert!(self.last.is_null());
            self.last = new;
        }
        else {
            debug_assert!(!self.last.is_null());
            unsafe { (*self.first).prev = new; }
        }
        // Make this the first node.
        self.first = new;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.first.is_null() {
            None
        } else {
            let first = self.first;
            let new_first = unsafe { (*self.first).next };
            self.first = new_first;
            if new_first.is_null() {
                // The list is now empty.
                self.last = new_first;
            } else {
                unsafe { (*new_first).prev = ptr::null_mut() };
            }
            let first = unsafe { raw_into_box(first) } ;
            Some(first.data)
        }
    }

    pub fn for_each<F: FnMut(&mut T)>(&mut self, mut f: F) {
        let mut cur_ptr = self.first;
        while !cur_ptr.is_null() {
            // Iterate over every node, and call `f`.
            f(unsafe{ &mut (*cur_ptr).data });
            cur_ptr = unsafe{ (*cur_ptr).next };
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.first, _marker: PhantomData  }
    }
}

pub struct IterMut<'a, T> where T: 'a {
    next: NodePtr<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
           None
        } else {
            let ret = unsafe{ &mut (*self.next).data };
            self.next = unsafe { (*self.next).next };
            Some(ret)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_ptr = self.first;
        while !cur_ptr.is_null() {
            let cur = unsafe { raw_into_box(cur_ptr) };
            cur_ptr = cur.next;
            drop(cur);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::Cell;
    use super::LinkedList;

    #[test]
    fn test_pop_back() {
        let mut l: LinkedList<i32> = LinkedList::new();
        for i in 0..3 {
            l.push_front(-i);
            l.push_back(i);
        }

        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.pop_back(), Some(1));
        assert_eq!(l.pop_back(), Some(0));
        assert_eq!(l.pop_back(), Some(-0));
        assert_eq!(l.pop_back(), Some(-1));
        assert_eq!(l.pop_back(), Some(-2));
        assert_eq!(l.pop_back(), None);
        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut l: LinkedList<i32> = LinkedList::new();
        for i in 0..3 {
            l.push_front(-i);
            l.push_back(i);
        }

        assert_eq!(l.pop_front(), Some(-2));
        assert_eq!(l.pop_front(), Some(-1));
        assert_eq!(l.pop_front(), Some(-0));
        assert_eq!(l.pop_front(), Some(0));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), None);
        assert_eq!(l.pop_front(), None);
    }

    #[derive(Clone)]
    struct DropChecker {
        count: Rc<Cell<usize>>,
    }
    impl Drop for DropChecker {
        fn drop(&mut self) {
            self.count.set(self.count.get() + 1);
        }
    }

    #[test]
    fn test_drop() {
        let count = DropChecker { count: Rc::new(Cell::new(0)) };
        {
            let mut l = LinkedList::new();
            for _ in 0..10 {
                l.push_back(count.clone());
                l.push_front(count.clone());
            }
        }
        assert_eq!(count.count.get(), 20);
    }

    #[test]
    fn test_iter_mut() {
        let mut l = LinkedList::<i32>::new();
        for i in 0..5 {
            l.push_back(i);
        }

        assert_eq!(l.pop_front(), Some(0));
        assert_eq!(l.pop_back(), Some(4));

        for (n, i) in l.iter_mut().enumerate() {
            *i-=1;
            assert_eq!(n as i32, *i);
        }
    }
}
