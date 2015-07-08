// Rust-101, Part 09: Closures (WIP)
// =================================

use std::io::prelude::*;
use std::io;

use part05::BigInt;

trait Action {
    fn do_action(&mut self, digit: u64);
}

impl BigInt {
    fn act_v1<A: Action>(&self, mut a: A) {
        for digit in self {
            a.do_action(digit);
        }
    }
}

struct PrintWithString {
    prefix: String,
}

impl Action for PrintWithString {
    fn do_action(&mut self, digit: u64) {
        println!("{}{}", self.prefix, digit);
    }
}

fn print_with_prefix_v1(b: &BigInt, prefix: String) {
    let my_action = PrintWithString { prefix: prefix };
    b.act_v1(my_action);
}

pub fn main() {
    let bignum = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    print_with_prefix_v1(&bignum, "Digit: ".to_string());
}

impl BigInt {
    fn act<A: FnMut(u64)>(&self, mut a: A) {
        for digit in self {
            a(digit);
        }
    }
}

pub fn print_with_prefix(b: &BigInt, prefix: String) {
    b.act(|digit| println!("{}{}", prefix, digit) );
}

//@ [index](main.html) | [previous](part08.html) | [next](main.html)
