#![allow(unused_variables)]
#![allow(dead_code)]


fn main() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(v) | None
    let results = if y != 0.0 {Some(x/y)} else {None};

    match results {
        Some(z) => println!("{}/{} = {}", x,y,z),
        None => println!("cannot divide by zero")
    }

    // a simple way to use option and match
    if let Some(z) => result {
        println!("result = {}", z)
    }
    
}
