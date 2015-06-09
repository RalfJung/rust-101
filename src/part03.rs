// Rust-101, Part 03: Input, Formatting
// ====================================

use std::io::prelude::*;
use std::io;

fn read_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D.");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.parse::<i32>() {
            Ok(num) => vec.push(num),
            Err(_) => println!("What did I say about numbers?"),
        }
    }

    vec
}

enum SomethingOrNothing<T>  {
    Something(T),
    Nothing,
}
use self::SomethingOrNothing::{Something,Nothing};

trait Minimum : Copy {
    fn min(a: Self, b: Self) -> Self;
}

fn vec_min<T: Minimum>(v: &Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        let e = *e;
        min = Something(match min {
            Nothing => e,
            Something(n) => T::min(n, e)
        });
    }
    min
}

impl Minimum for i32 {
    fn min(a: Self, b: Self) -> Self {
        ::std::cmp::min(a, b)
    }
}

use std::fmt;
impl<T: fmt::Display> fmt::Display for SomethingOrNothing<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Something(ref t) => t.fmt(f),
            &Nothing => "Nothing".fmt(f),
        }
    }
}

pub fn part_main() {
    let vec = read_vec();
    let min = vec_min(&vec);
    println!("The minimum is: {}", min);
}

// [index](main.html) | [previous](part02.html) | next
