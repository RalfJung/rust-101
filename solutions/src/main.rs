extern crate docopt;

pub mod bigint;
pub mod vec;
pub mod rgrep;

pub fn main() {
    rgrep::main();
}