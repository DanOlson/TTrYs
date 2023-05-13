#![allow(dead_code)]
mod matrix;
mod piece;

use matrix::Matrix;

fn main() {
    let matrix = Matrix::b_type();

    println!("{matrix}");
}
