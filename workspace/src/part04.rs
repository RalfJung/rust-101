// Rust-101, Part 04: Ownership, Borrowing
// =======================================

/*
  void foo(std::vector<int> v) {
      int *first = &v[0];
      v.push_back(42);
      *first = 1337; // This is bad!
  }
*/

// ## Ownership
fn work_on_vector(v: Vec<i32>) { /* do something */ }
fn ownership_demo() {
    let v = vec![1,2,3,4];
    work_on_vector(v);
    /* println!("The first element is: {}", v[0]); */
}

// ## Shared borrowing

fn vec_min(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;
    for e in v {
        // In the loop, `e` now has type `&i32`, so we have to dereference it to obtain an `i32`.
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

// Now that `vec_min` does not acquire ownership of the vector anymore, we can call it multiple times on the same vector and also do things like
fn shared_borrow_demo() {
    let v = vec![5,4,3,2,1];
    let first = &v[0];
    vec_min(&v);
    vec_min(&v);
    println!("The first element is: {}", *first);
}

// ## Mutable borrowing

fn vec_inc(v: &mut Vec<i32>) {
    for e in v {
        *e += 1;
    }
}
// Here's an example of calling `vec_inc`.
fn mutable_borrow_demo() {
    let mut v = vec![5,4,3,2,1];
    /* let first = &v[0]; */
    vec_inc(&mut v);
    vec_inc(&mut v);
    /* println!("The first element is: {}", *first); */
}

// ## Summary
// The ownership and borrowing system of Rust enforces the following three rules:
// 
// * There is always exactly one owner of a piece of data
// * If there is an active mutable borrow, then nobody else can have active access to the data
// * If there is an active shared borrow, then every other active access to the data is also a shared borrow
// 
// As it turns out, combined with the abstraction facilities of Rust, this is a very powerful mechanism
// to tackle many problems beyond basic memory safety. You will see some examples for this soon.

