use part05::BigInt;

pub trait Minimum {
    /// Return the smaller of the two
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

/// Return a pointer to the minimal value of `v`.
pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => e.min(n)
        });
    }
    min
}
// Notice that `Option<&T>` is technically (leaving the borrowing story aside) a pointer to a `T`,
// that could optionally be invalid. In other words, it's just like a pointer in C(++) or Java
// that can be `NULL`! However, thanks to `Option` being an `enum`, we cannot forget to check
// the pointer for validity, avoiding the safety issues of C(++). At the same time, when we
// have a borrow like `v` above that's not an `Option`, we *know* that is has to be a valid
// pointer, so we don't even need to do a `NULL`-check.<br/>
// Also, if you are worried about wasting space, notice that Rust knows that `&T` can never be
// `NULL`, and hence optimizes `Option<&T>` to be no larger than `&T`. The `None` case is represented
// as `NULL`. This is another great example of a zero-cost abstraction: `Option<&T>` is exactly like
// a pointer in C(++), if you look at what happens during execution - but it's much safer to use.

impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // compare back-to-front, i.e., most significant digit first
            let mut idx = self.data.len()-1;
            while idx > 0 {
                if self.data[idx] < other.data[idx] {
                    return self;
                } else if self.data[idx] > other.data[idx] {
                    return other;
                }
                else {
                    idx = idx-1;
                }
            }
            // the two are equal
            return self;
        }
    }
}
