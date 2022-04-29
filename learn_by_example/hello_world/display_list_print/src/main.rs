use std::fmt;
use std::fmt::{Formatter, write}; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("Debug {:?}", v);
    println!("Display {}", v);
}
