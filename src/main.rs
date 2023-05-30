#![allow(dead_code)]
mod matrix;
mod piece;

use matrix::Matrix;

fn main() {
    let matrix = Matrix::random_partial_fill();

    println!("{matrix}");
}
