use std::cmp;
use crate::piece::{Piece, Point, Shape};
use crate::matrix::Matrix;

fn debug_print_matrix(matrix: &Matrix<usize>) {
    matrix.rows.iter().rev().for_each(|r| println!("{:?}", r));
    println!("\n");
}

// From a given piece, return a new piece containing
// the points after rotation.
pub fn rotate_clockwise(piece: &Piece) -> Piece {
    let bbox = generate_bounding_matrix(piece);
    debug_print_matrix(&bbox);

    let mut transposed = transpose(bbox);

    reverse_rows(&mut transposed);
    debug_print_matrix(&transposed);

    let (lower_left, upper_right) = piece.bounds();
    let mut points: Vec<Point> = vec![];
    for (y, row) in transposed.rows.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val > 0 {
                let new_x = x + lower_left.x;
                let new_y = y + lower_left.y;
                points.push(Point::new(new_x, new_y));
            }
        }
    }
    let points: [Point; 4] = points
        .as_slice()
        .try_into()
        .unwrap();
    Piece::new(piece.shape, points)
}

pub fn rotate_counterclockwise(piece: &Piece) -> Piece {
    todo!()
}

fn reverse_rows(matrix: &mut Matrix<usize>) {
    matrix
        .rows
        .iter_mut()
        .for_each(|row| row.reverse());
}

fn generate_bounding_matrix(piece: &Piece) -> Matrix<usize> {
    let (lowest, highest) = piece.bounds();
    let width = highest.x - lowest.x + 1;
    let height = highest.y - lowest.y + 1;
    let len = cmp::max(width, height);
    let mut out = Matrix::square(len, 0);
    piece.points.iter().enumerate().for_each(|(i, p)| {
        let x = p.x - lowest.x;
        let y = p.y - lowest.y;
        out.set(x, y, i + 1);
    });
    out
}

fn transpose(matrix: Matrix<usize>) -> Matrix<usize> {
    let height = matrix.height();
    let width = matrix.width();
    let mut out = Matrix::new(width, height, 0);

    for y in 0..height {
        for x in 0..width {
            let val = matrix.get(x, y).unwrap();
            let new_x = width - x - 1; // return to zero-based
            let new_y = height - y - 1;
            out.set(new_y, new_x, val);
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_clockwise_orange_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::orange_ricky(origin);
        let r90 = rotate_clockwise(&piece);

        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(6, 12),
            Point::new(7, 12),
        ]);

        let r180 = rotate_clockwise(&r90);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12),
        ]);

        let r270 = rotate_clockwise(&r180);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(7, 12),
        ])
    }

    #[test]
    fn test_generate_bounding_matrix_orange_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::orange_ricky(origin);
        let matrix = generate_bounding_matrix(&piece);
        assert_eq!(matrix.width(), 3);
        assert_eq!(matrix.height(), 3);
        assert_eq!(matrix.get(0, 0).unwrap(), 1);
        assert_eq!(matrix.get(1, 0).unwrap(), 0);
        assert_eq!(matrix.get(2, 0).unwrap(), 0);
        assert_eq!(matrix.get(0, 1).unwrap(), 2);
        assert_eq!(matrix.get(1, 1).unwrap(), 3);
        assert_eq!(matrix.get(2, 1).unwrap(), 4);
        assert_eq!(matrix.get(0, 2).unwrap(), 0);
        assert_eq!(matrix.get(1, 2).unwrap(), 0);
        assert_eq!(matrix.get(2, 2).unwrap(), 0);
    }

    #[test]
    fn test_transpose() {
        let mut matrix = Matrix::new(3, 3, 0);

        let mut val = 0;
        for y in 0..=2 {
            for x in 0..=2 {
                val += 1;
                matrix.set(x, y, val);
            }
        }

        assert_eq!(matrix.get(0, 0).unwrap(), 1);
        assert_eq!(matrix.get(1, 0).unwrap(), 2);
        assert_eq!(matrix.get(2, 0).unwrap(), 3);
        assert_eq!(matrix.get(0, 1).unwrap(), 4);
        assert_eq!(matrix.get(1, 1).unwrap(), 5);
        assert_eq!(matrix.get(2, 1).unwrap(), 6);
        assert_eq!(matrix.get(0, 2).unwrap(), 7);
        assert_eq!(matrix.get(1, 2).unwrap(), 8);
        assert_eq!(matrix.get(2, 2).unwrap(), 9);
        debug_print_matrix(&matrix);

        let transposed = transpose(matrix);
        debug_print_matrix(&transposed);

        assert_eq!(transposed.get(0, 0).unwrap(), 9);
        assert_eq!(transposed.get(1, 0).unwrap(), 6);
        assert_eq!(transposed.get(2, 0).unwrap(), 3);
        assert_eq!(transposed.get(0, 1).unwrap(), 8);
        assert_eq!(transposed.get(1, 1).unwrap(), 5);
        assert_eq!(transposed.get(2, 1).unwrap(), 2);
        assert_eq!(transposed.get(0, 2).unwrap(), 7);
        assert_eq!(transposed.get(1, 2).unwrap(), 4);
        assert_eq!(transposed.get(2, 2).unwrap(), 1);
    }
}
