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

fn read_one_line() -> String {
    println!("Please enter a line of text.");
    let mut stdin = io::stdin();
    let mut prefix = "".to_string();
    stdin.read_line(&mut prefix).unwrap();
    prefix
}

pub fn main_v1() {
    let prefix = read_one_line();
    let my_action = PrintWithString { prefix: prefix };
    let bignum = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    bignum.act_v1(my_action);
}

impl BigInt {
    fn act<A: FnMut(u64)>(&self, mut a: A) {
        for digit in self {
            a(digit);
        }
    }
}

pub fn main() {
    let prefix = read_one_line();
    let bignum = BigInt::new(1 << 63) + BigInt::new(1 << 16) + BigInt::new(1 << 63);
    bignum.act(|digit| println!("{}{}", prefix, digit) );
}

