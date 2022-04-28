#[allow(unused_variables)]

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a={:?}", a);

    a.push(44);
    println!("a={:?}", a);

    // the following i32 type idx doesn't work in rust due to safety reason
    // idx cannot be negative
    // let idx:i32 = 0;

    // usize works as idx
    let idx:usize = 0;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);
    
    // Option
    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element")
    }

    for i in &a {
        println!("{}", i);
    }

    a.push(44);
    println!("{:?}", a);

    // Option
    // last elem is Some(44), not 44
    let last_element = a.pop();
    println!("last elem is {:?}, a={:?}", last_element, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn main() {
    vectors();
}
