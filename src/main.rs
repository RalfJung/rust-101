#![allow(dead_code)]
// Welcome to Rust-101
// ===================
//
// This is Rust-101, a small tutorial to the [Rust language](http://www.rust-lang.org/).
// This is intended to be an interactive, hands-on course: I believe the only way to
// *really* learn a language is to write code in it, so you should be coding during
// the course. These documents mainly serve as a guide to the teacher, reminding me
// what to explain in which order, and making sure I have sample code for all topics
// I plan to cover. They may also be helpful as an offline resource, but you're on your
// own then.
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
// [the second chapter of The Book](https://doc.rust-lang.org/stable/book/getting-started.html).
// When you are done, running `cargo build` in the root of Rust-101 should successfully compile
// all the code.
//
// Course Content
// --------------
// 
// The actual course is in the partXX.rs files. I suggest you get started with
// [the first part](part00.html), or jump directly to where you left off:
// 
// * [Part 00](part00.html)
// * [Part 01](part01.html)
mod part00;
mod part01;

// To actually run the code after filling in the blanks, simply edit the `main`
// function below.

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
