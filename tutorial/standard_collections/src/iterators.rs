#[allow(unused_variables)]

pub fn main() {
    let mut vec = vec![3, 2, 1];
    
    for x in &vec {
        println!("{}", x);
    }

    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    let mut vec2 = vec![1, 2, 3];

    //let it = vec.into_iter();
    vec2.extend(vec);
    println!("{:?}",vec2);
}
