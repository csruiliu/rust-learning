#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::{Add, AddAssign, Neg};
use std::cmp::PartialEq;

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im:T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}


impl<T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Complex<T>;
    
    // a+b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }   
}


impl<T> Neg for Complex<T> where T: Neg<Output = T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im 
        }
    }
}

impl<T> PartialEq for Complex<T> where T:PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

fn main() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);

    //a += b;
    println!("{:?}", a==a);
}