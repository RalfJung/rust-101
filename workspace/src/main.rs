mod part00;
mod part01;
mod part02;
mod part04;

#[cfg(not(test))] /* If you get warnings about functions not being used on "crate test", adding this attribute will fix them.
                     It says that the function is only to be compiled if we are *not* compiling for tests. */
fn main() {
   part00::main();
}
