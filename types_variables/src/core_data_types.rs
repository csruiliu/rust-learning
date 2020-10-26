#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let a: u8 = 123; //u8 = unsigned, 8 bits
    println!("a={}", a); // immutable variable

    // u = unsigned, 0 to 2^n-1
    // i = signed, -2^(n-1) to 2^(n-1)-1
    let mut b: i8 = 0; // mutable variable
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);


    //implicitly define a variable (immutable)
    let c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // u8, u16, u32, u64, i8, i16, ...
    // uszie, isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z={}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));
    
    // f32, f64
    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    // default type is f64
    let f = 2.5;
    println!("{}, size = {} bytes", f, mem::size_of_val(&f));

    // bool variable
    let g:bool = false;
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
}   