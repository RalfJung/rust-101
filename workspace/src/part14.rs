// Rust-101, Part 14: Slices, Arrays, External Dependencies
// ========================================================


// ## Slices

pub fn sort<T: PartialOrd>(data: &mut [T]) {
    if data.len() < 2 { return; }

    // We decide that the element at 0 is our pivot, and then we move our cursors through the rest of the slice,
    // making sure that everything on the left is no larger than the pivot, and everything on the right is no smaller.
    let mut lpos = 1;
    let mut rpos = data.len();
    /* Invariant: pivot is data[0]; everything with index (0,lpos) is <= pivot;
       [rpos,len) is >= pivot; lpos < rpos */
    loop {
        // **Exercise 14.1**: Complete this Quicksort loop. You can use `swap` on slices to swap two elements. Write a
        // test function for `sort`.
        unimplemented!()
    }

    // Once our cursors met, we need to put the pivot in the right place.
    data.swap(0, lpos-1);

    // Finally, we split our slice to sort the two halves. The nice part about slices is that splitting them is cheap:
    let (part1, part2) = data.split_at_mut(lpos);
    unimplemented!()
}

// **Exercise 14.2**: Since `String` implements `PartialEq`, you can now change the function `output_lines` in the previous part
// to call the sort function above. If you did exercise 13.1, you will have slightly more work. Make sure you sort by the matched line
// only, not by filename or line number!

// Now, we can sort, e.g., an vector of numbers.
fn sort_nums(data: &mut Vec<i32>) {
    sort(&mut data[..]);
}

// ## Arrays
fn sort_array() {
    let mut array_of_data: [f64; 5] = [1.0, 3.4, 12.7, -9.12, 0.1];
    sort(&mut array_of_data);
}

// ## External Dependencies


// I disabled the following module (using a rather bad hack), because it only compiles if `docopt` is linked.
// Remove the attribute of the `rgrep` module to enable compilation.
#[cfg(feature = "disabled")]
pub mod rgrep {
    // Now that `docopt` is linked, we can first add it to the namespace with `extern crate` and then import shorter names with `use`.
    // We also import some other pieces that we will need.
    extern crate docopt;
    use self::docopt::Docopt;
    use part13::{run, Options, OutputMode};
    use std::process;

    // The `USAGE` string documents how the program is to be called. It's written in a format that `docopt` can parse.
    static USAGE: &'static str = "
Usage: rgrep [-c] [-s] <pattern> <file>...

Options:
    -c, --count  Count number of matching lines (rather than printing them).
    -s, --sort   Sort the lines before printing.
";

    // This function extracts the rgrep options from the command-line arguments.
    fn get_options() -> Options {
        // This parses `argv` and exit the program with an error message if it fails. The code is taken from the [`docopt` documentation](http://burntsushi.net/rustdoc/docopt/). <br/>
        let args = Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit());
        // Now we can get all the values out.
        let count = args.get_bool("-c");
        let sort = args.get_bool("-s");
        let pattern = args.get_str("<pattern>");
        let files = args.get_vec("<file>");
        if count && sort {
            println!("Setting both '-c' and '-s' at the same time does not make any sense.");
            process::exit(1);
        }

        // We need to make the strings owned to construct the `Options` instance.
        let mode = if count {
            OutputMode::Count
        } else if sort {
            OutputMode::SortAndPrint
        } else {
            OutputMode::Print
        };
        Options {
            files: files.iter().map(|file| file.to_string()).collect(),
            pattern: pattern.to_string(),
            output_mode: mode,
        }
    }

    // Finally, we can call the `run` function from the previous part on the options extracted using `get_options`. Edit `main.rs` to call this function.
    // You can now use `cargo run -- <pattern> <files>` to call your program, and see the argument parser and the threads we wrote previously in action!
    pub fn main() {
        unimplemented!()
    }
}

// **Exercise 14.3**: Wouldn't it be nice if rgrep supported regular expressions? There's already a crate that does all the parsing and matching on regular
// expression, it's called [regex](https://crates.io/crates/regex). Add this crate to the dependencies of your workspace, add an option ("-r") to switch
// the pattern to regular-expression mode, and change `filter_lines` to honor this option. The documentation of regex is available from its crates.io site.
// (You won't be able to use the `regex!` macro if you are on the stable or beta channel of Rust. But it wouldn't help for our use-case anyway.)

