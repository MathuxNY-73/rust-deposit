use ndarray::prelude::*;

fn main() {
    let test_array = array![[1.,2.,3.], [4.,5.,6.]];
    let other = array![[2.,3.,4.], [5.,6.,7.]];

    let dot = &test_array * &other;
    let muti = test_array.dot(&other.t());

    for x in dot.iter() {
        println!("{:.2}", x);
    }

    println!("Multiplication:");

    for x in muti.iter() {
        println!("{:.2}", x);
    }

    println!("Hello, world!");
}
