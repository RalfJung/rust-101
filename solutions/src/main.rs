// This crate contains solutions to *some* of the exercises, and it bundles
// the projects that span multiple parts together in one file per project.
// It is not always up-to-date with the code in the actual course, and mainly
// serves as draft board for new parts or exercises.

extern crate docopt;

pub mod bigint;
pub mod vec;
pub mod rgrep;
pub mod callbacks;
pub mod counter;
pub mod list;

pub fn main() {
    rgrep::main();
}