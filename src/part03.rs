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

fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
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

impl SomethingOrNothing<i32> {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}
pub fn part_main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}

impl SomethingOrNothing<i32> {
    fn equals(self, other: Self) -> bool {
        match (self, other) {
            (Nothing     , Nothing      ) => true,
            (Something(n), Something (m)) => n == m,
            _ => false,
        }
    }
}

#[test]
fn tes_vec_min() {
    assert!(vec_min(vec![6,325,33,532,5,7]).equals(Something(5)));
    assert!(vec_min(vec![]).equals(Nothing));
}

// [index](main.html) | [previous](part02.html) | next
