// Welcome to Rust-101
// ===================
// 
// This is [Rust-101](https://www.ralfj.de/projects/rust-101/), a small *work-in-progress*
// tutorial for the [Rust language](http://www.rust-lang.org/).
// It is intended to be an interactive, hands-on course: I believe the only way to
// *really* learn a language is to write code in it, so you should be coding during
// the course. I am writing this tutorial with a tutorial situation in mind, i.e.,
// with a teacher being around to guide students through the course and answer
// questions as they come up. However, I think they may also be useful if you
// work through them on your own. Just make sure to actually play with the code.
// If you have any questions, maybe the "Additional Resources" below are useful.
// 
// I will assume basic familiarity with programming, and hence not explain the basic
// concepts common to most languages. Instead, I will focus on what makes Rust special.
// 
// Prerequisites
// -------------
// 
// You will need to have Rust installed, of course. It is available for download on
// [the Rust website](http://www.rust-lang.org/). At this point, I plan to restrict
// myself to stable Rust, so "Recommended" version is just right.
// You can find some more installation instructions in
// [the second chapter of The Book](https://doc.rust-lang.org/stable/book/installing-rust.html).
// When you are done, running `cargo build` in the root of Rust-101 should successfully compile
// all the code.
// 
// Getting the source
// ------------------
// 
// You are meant to play around with the source code of the course as you go on, so please
// fetch it from the [git repository](http://www.ralfj.de/git/rust-101.git) (also available
// [on GitHub](https://github.com/RalfJung/rust-101)).
// 
// Course Content
// --------------
// 
// The actual course is in the partXX.rs files. I suggest you get started with
// [the first part](part00.html), or jump directly to where you left off:
// 
// * [Part 00](part00.html)
// * [Part 01](part01.html)
// * [Part 02](part02.html)
// * (to be continued)
#![allow(dead_code, unused_imports, unused_variables)]
mod part00;
mod part01;
mod part02;

// To actually run the code of some part (after filling in the blanks, if necessary), simply edit the `main`
// function.

fn main() {
    part00::part_main();
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
