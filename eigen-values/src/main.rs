extern crate nalgebra as na;
extern crate rand;

use rand::prelude::*;
use na::{DMatrix, linalg};

fn main() {
    let mut rng = thread_rng();
    let a = DMatrix::<f64>::from_fn(10, 10, |_i, _j| rng.gen::<f64>());
    println!("{}", a[(0,0)]);
    //println!("{}", a.new_random());
    let at = a.transpose();
    let b = a*at;
    let eig = linalg::SymmetricEigen::new(b);
    println!("Eigenvalues: {}", eig.eigenvalues);
}
