// Welcome to Rust-101
// ===================
// 
// This is [Rust-101](https://www.ralfj.de/projects/rust-101/), a small *work-in-progress*
// tutorial for the [Rust language](http://www.rust-lang.org/).
// It is intended to be an interactive, hands-on course: I believe the only way to
// *really* learn a language is to write code in it, so you should be coding during
// the course. I am writing this with a tutorial situation in mind, i.e.,
// with a teacher being around to guide students through the course and answer
// questions as they come up. However, I think they may also be useful if you
// work through them on your own, you will just have to show more initiative yourself:
// Make sure you actually type some code. It may sound stupid to manually copy code
// that you could duplicate through the clipboard, but it's actually helpful.
// If you have questions, check out the "Additional Resources" below. In particular,
// the IRC channel is filled with awesome people willing to help you! I spent
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
// languages sacrificies one of these goals for the other. In particular, the
// first requirement rules out a garbage collector: Rust can run "mare metal".
// In fact, Rust rules out more classes of bugs than languages that achieve safety
// with a GC: Besides dangling pointers and double-free, Rust also prevents issues
// such as iterator invalidation and race conditions.
// 
// 
// Getting started
// ---------------
// 
// You will need to have Rust installed, of course. It is available for download on
// [the Rust website](http://www.rust-lang.org/). At this point, I plan to restrict
// myself to stable Rust, so "Recommended" version is just right.
// You can find some more installation instructions in
// [the second chapter of The Book](https://doc.rust-lang.org/stable/book/installing-rust.html).

// Next, fetch the Rust-101 source code from the [git repository](http://www.ralfj.de/git/rust-101.git)
// (also available [on GitHub](https://github.com/RalfJung/rust-101)). Running `cargo build`
// in the root of the repository should now succeed.
// 
// I suggest you copy the folder `workspace` somewhere you like, so that you can still easily
// `git pull` updates in the main repository. After copying, try `cargo build` in the
// new location. It should complain about `part00::main()` not being found. Now you can start
// by following [Part 00](part00.html), typing in `part00.rs`. If you need to add a new file, remember to also
// add it to `main.rs` so that Rust finds it.
// 
// If you do not want to copy all the code yourself, and wish to start with my code and just edit
// it, you can copy the files from `src` in this repository, to `src` in your workspace copy.

// Course Content
// --------------
// 
// The actual course is in the partXX.rs files. The part 00-03 cover some basic of the language,
// to give you a feeling for Rust's syntax and pervasive mechanisms like pattern matching and traits.
// Parts 04-06 introduce the heart of the language, the mechanism making it different from anything
// else out there: Ownership, borrowing, lifetimes. In part 07-??, we continue our tour through
// Rust. Finally, in parts ??-??, we implement our own version of `grep`, exhibiting useful Rust
// features as we go.
// 
// You should start with [the first part](part00.html), or jump directly to where you left off:
// 
// * [Part 00: Algebraic datatypes](part00.html)
// * [Part 01: Expressions, Inherent methods](part01.html)
// * [Part 02: Generic types, Traits](part02.html)
// * [Part 03: Input](part03.html)
// * [Part 04: Ownership, Borrowing](part04.html)
// * [Part 05: Clone](part05.html)
// * [Part 06: Copy, Lifetimes](part06.html)
// * [Part 07: Operator Overloading, Tests, Formating](part07.html)
// * (to be continued)
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
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
