#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Circle {
    radius: f64
}

struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

fn main() {
    let shapes:[&Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }
}