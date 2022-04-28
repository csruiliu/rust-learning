fn main() {
    let v = vec![1,2,3];
    
    let v2 = v;

    // reference data
    // error: ownership has been transferred to v2
    // println!("{:?}", v);

    let u = 1;
    let u2 = u;
    
    // real data
    // full copy happen here, so can still use u   
    println!("u = {}", u);

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        return x;
    };

    let vv = print_vector(v2);
    println!("{}", vv[0]);
}