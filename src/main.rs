// Welcome to Rust-101
// ===================
// 
// This is [Rust-101](https://www.ralfj.de/projects/rust-101/), a small *work-in-progress*
// tutorial for the [Rust language](http://www.rust-lang.org/).
// It is intended to be an interactive, hands-on course: I believe the only way to
// *really* learn a language is to write code in it, so you should be coding during
// the course.
// 
// If you have any questions that are not answered here, check out the "Additional Resources"
// below. In particular, the IRC channel is filled with awesome people willing to help you! I spent
// lots of time there ;-)
// 
// I will assume some familiarity with programming, and hence not explain the basic
// concepts common to most languages. Instead, I will focus on what makes Rust special.
//
// Why Rust?
// ---------
// 
// When you got here, I am kind of assuming that you already decided to give Rust at
// least a look, so that I don't have to do much convincing here ;-) . But just in
// case, here's why I think Rust is worth learning:<br/>
// At this time, Rust is a language with a pretty unique set of goals. Rust aims to
// achieve C++-style control over memory and execution behavior (like, static vs. dynamic
// dispatch), which makes it possible to construct abstractions that carry no run-time
// cost. This is combined with the comfort of high-level functional languages and guaranteed
// safety (as in, the program will not crash). The vast majority of existing
// languages sacrifices one of these goals for the other. In particular, the
// first requirement rules out a garbage collector: Rust can run "bare metal".
// In fact, Rust rules out more classes of bugs than languages that achieve safety
// with a GC: Besides dangling pointers and double-free, Rust also prevents issues
// such as iterator invalidation and data races.
// 
// 
// Getting started
// ---------------
// 
// You will need to have Rust installed, of course. It is available for download on
// [the Rust website](http://www.rust-lang.org/). You should go for either the "stable"
// or the "beta" channel. More detailed installation instructions are provided in
// [the second chapter of The Book](https://doc.rust-lang.org/stable/book/installing-rust.html).
// This will also install `cargo`, the tool responsible for building rust projects (or *crates*).

// Next, fetch the Rust-101 source code from the [git repository](http://www.ralfj.de/git/rust-101.git)
// (also available [on GitHub](https://github.com/RalfJung/rust-101), and as a
// [zip archive](https://github.com/RalfJung/rust-101/archive/master.zip) in case you don't have git installed).
// 
// There is a workspace prepared for you in the `workspace` folder. I suggest you copy this
// folder somewhere else - that will make it much easier to later update the course without
// overwriting your changes. Try `cargo build` in that new folder to check that compiling your workspace succeeds.
// (You can also execute it with `cargo run`, but you'll need to do some work before this will succeed.)
// 
// If you later want to update the course, do `git pull` (or re-download the zip archive).
// Then copy the files from `workspace/src/` to your workspace that you did not yet work on. Definitely
// copy `main.rs` to make sure all the new files are actually compiled. (Of course you can also
// copy the rest, but that would replace all your hard work by the original files with all the holes!)

// Course Content
// --------------
// 
// The part 00-03 cover some basic of the language, to give you a feeling for Rust's syntax and pervasive
// mechanisms like pattern matching and traits. Parts 04-06 introduce the heart of the language, the ideas
// making it different from anything else out there: Ownership, borrowing, lifetimes. In part 07-??, we
// continue our tour through Rust with another example. Finally, in parts ??-??, we implement our own
// version of `grep`, exhibiting some more Rust features as we go.
// 
// Now, open `your-workspace/src/part00.rs` in your favorite editor, and follow the link below for
// the explanations and exercises. Have fun!
// 
// * [Part 00: Algebraic datatypes](part00.html)
// * [Part 01: Expressions, Inherent methods](part01.html)
// * [Part 02: Generic types, Traits](part02.html)
// * [Part 03: Input](part03.html)
// * [Part 04: Ownership, Borrowing](part04.html)
// * [Part 05: Clone](part05.html)
// * [Part 06: Copy, Lifetimes](part06.html)
// * [Part 07: Operator Overloading, Tests, Formating](part07.html)
// * [Part 08: Associated Types, Modules](part08.html)
// * [Part 09: Iterators](part09.html)
// * [Part 10: Closures](part10.html)
// * [Part 11: Trait Objects, Box, Rc, Lifetime bounds](part11.html)
// * [Part 12: Concurrency, Send](part12.html)
// * [Part 13: Slices, Arrays, External Dependencies](part13.html)
// * (to be continued)
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]
/* extern crate docopt; */
mod part00;
mod part01;
mod part02;
mod part03;
mod part04;
mod part05;
mod part06;
mod part07;
mod part08;
mod part09;
mod part10;
mod part11;
mod part12;
mod part13;

// To actually run the code of some part (after filling in the blanks, if necessary), simply edit the `main`
// function.
fn main() {
    part00::main();
}


// Additional material
// -------------------
// 
// There's tons of useful Rust stuff out there, so let me just put links to some
// of the most interesting places here:
// 
// * [The Rust Book](https://doc.rust-lang.org/stable/book/)
// * [Rust by Example](http://rustbyexample.com/)
// * The [Rust Subreddit](https://www.reddit.com/r/rust/)
// * For the IRC channel and other forums, see the "Community" section of the [Rust Documentation index](http://doc.rust-lang.org/index.html)
