pub mod part01 {
    use std;

    /// A number, or nothing
    pub enum NumberOrNothing {
        Number(i32),
        Nothing
    }
    use self::NumberOrNothing::{Number,Nothing};

    /// Compute the minimum element of the vector
    pub fn vec_min(v: Vec<i32>) -> NumberOrNothing {
        let mut min = Nothing;
        for e in v {
            min = Number(match min {
                Nothing => e,
                Number(n) => std::cmp::min(n, e)
            });
        }
        min
    }

    /// Compute the sum of elements in the vector
    pub fn vec_sum(v: Vec<i32>) -> i32 {
        let mut sum = 0;
        for e in v {
            sum += e;
        }
        sum
    }

    /// Print all elements in the vector
    pub fn vec_print(v: Vec<i32>) {
        for e in v {
            println!("{}", e)
        }
    }
}

pub mod part02 {
    // A polymorphic (generic) "some value, or no value"
    pub enum SomethingOrNothing<T>  {
        Something(T),
        Nothing,
    }
    pub use self::SomethingOrNothing::*;
    type NumberOrNothing = SomethingOrNothing<i32>;

    /// This trait is used to compute the minimum of two elements of the given type
    pub trait Minimum : Copy {
        fn min(self, b: Self) -> Self;
    }

    /// Return the minimum element of the vector
    pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
        let mut min = Nothing;
        for e in v {
            min = Something(match min {
                Nothing => e,
                Something(n) => e.min(n)
            });
        }
        min
    }

    /// We can compute the minimum of two integers
    impl Minimum for i32 {
        fn min(self, b: Self) -> Self {
            if self < b { self } else { b }
        }
    }

    /// Sample program to call vec_min
    impl NumberOrNothing {
        pub fn print(self) {
            match self {
                Nothing => println!("The number is: <nothing>"),
                Something(n) => println!("The number is: {}", n),
            };
        }
    }
    fn read_vec() -> Vec<i32> {
        vec![18,5,7,3,9,27]
    }
    pub fn main_i32() {
        let vec = read_vec();
        let min = vec_min(vec);
        min.print();
    }

    // Now, all the same for calling it on f32
    impl Minimum for f32 {
        fn min(self, b: Self) -> Self {
            if self < b { self } else { b }
        }
    }

    impl SomethingOrNothing<f32> {
        pub fn print_f32(self) {
            match self {
                Nothing => println!("The number is: <nothing>"),
                Something(n) => println!("The number is: {}", n),
            };
        }
    }

    fn read_vec_f32() -> Vec<f32> {
        vec![18.01,5.2,7.1,3.,9.2,27.123]
    }
    pub fn main_f32() {
        let vec = read_vec_f32();
        let min = vec_min(vec);
        min.print_f32();
    }

    /// Add a `Display` implementation to `SomethingOrNothing`
    use std::fmt;
    impl<T: fmt::Display> fmt::Display for SomethingOrNothing<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Something(ref t) => t.fmt(f),
                Nothing          => "Nothing".fmt(f),
            }
        }
    }
}

pub mod part03 {
    use std::io::prelude::*;
    use std::io;

    fn read_vec() -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::<i32>::new();
        let stdin = io::stdin();
        println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            match line.trim().parse::<i32>() {
                Ok(num) => {
                    vec.push(num)
                },
                // We don't care about the particular error, so we ignore it with a `_`.
                Err(_) => {
                    println!("What did I say about numbers?")
                },
            }
        }

        vec
    }

    use super::part02::{SomethingOrNothing,Something,Nothing,vec_min};

    pub fn main() {
        let vec = read_vec();
        let min = vec_min(vec);
        min.print2();
    }

    pub trait Print {
        fn print(self);
    }
    impl Print for i32 {
        fn print(self) {
            print!("{}", self);
        }
    }

    impl<T: Print> SomethingOrNothing<T> {
        fn print2(self) {
            match self {
                Nothing => println!("The number is: <nothing>"),
                Something(n) => {
                    print!("The number is: ");
                    n.print();
                    println!();
                }
            }
        }
    }

    impl Print for f32 {
        fn print(self) {
            print!("{}", self);
        }
    }
}
