extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let b:u16 = rng.gen();
    println!("{}", b); 

    println!("Integer: {}", rng.gen_range(0, 10));
}
