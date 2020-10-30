#[allow(dead_code)]
#[allow(unused_variables)]

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123;

fn main() {

    // mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
    // use unsafe block to do that, but need to be carefully
    unsafe {
        Z = 777;
        println!("Z: {}", Z);
    }

    println!("meaing of the life: {}", MEANING_OF_LIFE);
}   