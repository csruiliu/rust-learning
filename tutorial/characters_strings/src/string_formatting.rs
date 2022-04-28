#![allow(dead_code)]
#![allow(unused_must_use)]

fn main() {
    let name = "Rust";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let hello_name = format!("{}, {}!", hello, name);
    println!("{}", hello_name);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0},{1},{0}", run, forest);
    println!("{}", rfr);

    let info = format!("the name's {last}. {first} {last}.", first = "james", last = "bond");
    println!("{}", info);

    let mixed = format!("{1} {} {0} {data}", "alpha", "beta", data="delta");
    println!("{}", mixed);
}
