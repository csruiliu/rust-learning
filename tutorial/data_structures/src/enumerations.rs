#![allow(unused_variables)]
#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // tuple-style
    CMYKColor{cyan:u8, magenta:u8, yellow:u8, black:u8} // struct-style
}

fn enums() {
    //let c:Color = Color::Red;
    //let c:Color = Color::RGBColor(0,10,0);
    let c:Color = Color::CMYKColor{cyan:0, magenta:128, yellow:0, black:255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0, 0, 0) => println!("black"),
        Color::RGBColor(r,g,b) => println!("rbg({},{},{})",r,g,b),
        Color::CMYKColor{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        _ => ()
    }
}

fn main() {
    enums()
}
