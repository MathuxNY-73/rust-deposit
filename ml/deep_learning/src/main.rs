use ndarray::prelude::*;

fn transform_point_cloud(point_cloud: Array2<f64>, translation: &Array2<f64>, rotation: &Array2<f64>) -> Array2<f64> {
    let mut results = point_cloud.dot(&translation.t());
    results += rotation;

    results
}

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
