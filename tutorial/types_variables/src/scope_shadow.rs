#[allow(dead_code)]
#[allow(unused_variables)]

fn scope_and_shadowing() {
    // only available in this scope
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);
        
        // shadow the outside variable a
        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
}


fn main() {
    scope_and_shadowing()
}   