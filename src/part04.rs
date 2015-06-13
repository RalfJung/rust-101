// Rust-101, Part 04: Ownership, Borrowing
// =======================================

use std::cmp;

// Rust aims to be a "safe systems language". As a systems language, of course it
// provides *references* (or *pointers*). But as a safe language, it has to
// prevent bugs like this C++ snippet.
/*
  void foo(std::vector<int> v) {
      int &first = v[0];
      v.push_back(42);
      first = 1337; // This is bad!
  }
*/
// What's going wrong here? `first` is a reference into the vector `v`.
// The operation `push_back` may re-allocate the storage for the vector,
// in case the old buffer was full. If that happens, `first` is now
// a dangling pointer, and accessing it can crash the program (or worse).
// 
// It turns out that only the combination of two circumstances can lead to such a bug:
// *aliasing* and *mutation*. In the code above, we have `first` and the buffer of `v`
// being aliases, and when `push_back` is called, the latter is used to perform a mutation.
// Therefore, the central principle of the Rust typesystem is to *rule out mutation in the presence
// of aliasing*. The core tool to achieve that is the notion of *ownership*.

// What does that mean in practice? Consider the following example.
fn take(v: Vec<i32>) { /* do something */ }
fn foo1() {
    let v = vec![1,2,3,4];
    take(v);
    /* println!("The first element is: {}", v[0]); */
}
// Rust attaches additional meaning to the argument of `take`: The function can assume
// that it entirely *owns* `v`, and hence can do anything with it. When `take` ends,
// nobody needs `v` anymore, so it will be deleted (including its buffer on the heap).
// Passing a `Vec<i32>` to `take` is considered *transfer of ownership*: Someone used
// to own that vector, but now he gave it on to `take` and has no business with it anymore.
// 
// If you give a book to your friend, you cannot some to his place next day and get the book!
// It's no longer yours. Rust makes sure you don't break this rule. Try enabling the commented
// line in `foo1`. Rust will tell you that `v` has been *moved*, which is to say that ownership
// has been transferred somewhere else. In this particular case, the buffer storing the data
// does not even exist anymore, so we are lucky that Rust caught this problem!
// Essentially, ownership rules out aliasing, hence making the kind of problem discussed above
// impossible.

// If you go back to our example with `vec_min`, and try to call that function twice, you will
// get the same error. That's because `vec_min` demands that the caller transfers ownership of the
// vector. Hence, when `vec_min` finishes, the entire vector is deleted. That's of course not what
// we wanted! Can't we somehow give `vec_min` access to the vector, while retaining ownership of it?
// 
// Rust calls this *borrowing* the vector, and it works a bit like borrowing does in the real world:
// If you borrow a book to your friend, your friend can have it and work on it (and you can't!)
// as long as the book is still borrowed. Your friend could even borrow the book to someone else.
// Eventually however, your friend has to give the book back to you, at which point you again
// have full control.
// 
// Rust distinguishes between two kinds of borrows. First of all, there's the *shared* borrow.
// This is where the book metaphor kind of breaks down... you can give a shared borrow of
// *the same data* to lots of different people, who can all access the data. This of course
// introduces aliasing, so in order to live up to its promise of safety, Rust does not allow
// mutation through a shared borrow.

// So, let's re-write `vec_min` to work on a shared borrow of a vector. In fact, the only
// thing we have to change is the type of the function. The `e` in the loop now gets type
// `&i32`, hence we have to deference it.
fn vec_min(v: &Vec<i32>) -> Option<i32> {
    let mut min = None;
    for e in v {
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

// Now that `vec_min` does not acquire ownership of the vector anymore, we can call it multiple times on the same vector and also do things like
fn foo2() {
    let v = vec![5,4,3,2,1];
    let first = &v[0];
    vec_min(&v);
    vec_min(&v);
    println!("The first element is: {}", *first);
}
// What's going on here? First, `&` is how you create a shared borrow to something. This code creates three
// shared borrows to `v`: The borrow for `first` begins in the 2nd line of the function and lasts all the way to
// the end. The other two borrows, created for calling `vec_min`, only last for the duration of that
// call.
// 
// Technically, of course, borrows are pointers. Notice that since `vec_min` only gets a shared
// borrow, Rust knows that it cannot mutate `v` in any way. Hence the pointer created before calling
// `vec_min` remains valid.

// There is a second kind of borrow, a *mutable borrow*. As the name suggests, such a borrow permits
// mutation, and hence has to prevent aliasing. There can only ever be one mutable borrow to a
// particular piece of data.

// As an example, consider a function which increments every element of a vector by 1.
fn vec_inc(v: &mut Vec<i32>) {
    for e in v {
        *e += 1;
    }
}
// The type `&mut Vec<i32>` is the type of mutable borrows of `vec<i32>`. Because the borrow is
// mutable, we can change `e` in the loop. How can we call this function?
fn foo3() {
    let mut v = vec![5,4,3,2,1];
    /* let first = &v[0]; */
    vec_inc(&mut v);
    vec_inc(&mut v);
    /* println!("The first element is: {}", *first); */
}
// `&mut` is the operator to create a mutable borrow. We have to mark `v` as mutable in order
// to create such a borrow. Because the borrow passed to `vec_inc` only lasts as
// long as the function call, we can still call `vec_inc` on the same vector twice:
// The durations of the two borrows do not overlap, so we never have more than one mutable borrow.
// However, we can *not* create a shared borrow that spans a call to `vec_inc`. Just try
// enabling the commented-out lines. This is because `vec_inc` could mutate the vector structurally
// (i.e., it could add or remove elements), and hence the pointer `first` could become invalid.
// 
// Above, I said that having a mutable borrow excludes aliasing. But if you look at the code above carefully,
// you may say: "Wait! Don't the `v` in `foo3` and the `v` in `vec_inc` alias?" And you are right,
// they do. However, the `v` in `foo3` is not actually usable, it is not *active*: As long as there is an
// outstanding borrow, Rust will not allow you to do anything with `v`. This is, in fact, what
// prevents the creation of a mutable borrow when there already is a shared one.

// This also works the other way around: In `foo4`, there is already a mutable borrow active in the `vec_min`
// line, so the attempt to create another shared borrow is rejected by the compiler.
fn foo4() {
    let mut v = vec![5,4,3,2,1];
    let first = &mut v[0];
    /* vec_min(&v); */
    println!("The first element is: {}", *first);
}

// So, to summarize: The ownership and borrowing system of Rust enforces the following three rules:
// 
// * There is always exactly one owner of a piece of data
// * If there is an active mutable borrow, then nobody else can have active access to the data
// * If there is an active shared borrow, then every other active access to the data is also a shared borrow
// 
// As it turns out, combined with the abstraction facilities of Rust, this is a very powerful mechanism
// to tackle many problems beyond basic memory safety.

// [index](main.html) | [previous](part03.html) | [next](main.html)
