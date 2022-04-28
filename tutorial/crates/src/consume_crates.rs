extern crate rand;
extern crate modules_crates;

use rand::Rng;
use modules_crates::greetings::english;
use modules_crates::greetings::french;

fn main() {
    let mut rng = rand::thread_rng();
    let b:u16 = rng.gen();
    println!("{}", b); 

    println!("Integer: {}", rng.gen_range(0, 10));

    println!("English: {}, {}", english::hello(), english::goodbye());
    println!("French: {}, {}", french::hello(), french::goodbye());
}
