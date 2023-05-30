use rand::{thread_rng, Rng};
use crate::piece::{Piece, Point};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const EMPTY_ROW: [u8; WIDTH] = [0; WIDTH];

#[derive(PartialEq)]
pub struct Matrix {
    pub rows: [[u8; WIDTH]; HEIGHT]
}

impl Matrix {
    pub fn empty() -> Self {
        Matrix {
            rows: [EMPTY_ROW; HEIGHT]
        }
    }

    pub fn random_partial_fill() -> Self {
        let mut rows = [EMPTY_ROW; HEIGHT];
        for i in 0..HEIGHT {
            if i > HEIGHT - 6 {
                rows[i] = Self::random_row();
            }
        }
        Matrix { rows }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        Some(*self.rows.get(y)?.get(x)?)
    }

    pub fn set(&mut self, x: usize, y: usize) -> Option<u8> {
        let mut row = *self.rows.get(y)?;
        let prev = *row.get(x)?;
        row[x] = 1;
        self.rows[y] = row;
        Some(prev)
    }

    pub fn apply(&mut self, piece: Piece) -> Option<&Matrix> {
        if !self.can_apply(&piece) { return None }

        piece.points
            .iter()
            .for_each(|p| { self.set(p.x, p.y); });
        Some(self)
    }

    fn can_apply(&self, piece: &Piece) -> bool {
        piece.points
            .iter()
            .all(|p| self.get(p.x, p.y).eq(&Some(0)))
    }

    fn random_row() -> [u8; WIDTH] {
        let mut row = [0; WIDTH];
        for i in row.iter_mut().take(WIDTH) {
            *i = thread_rng().gen_bool(1.0 / 3.0).into();
        }
        row
    }
}

// +--+--+--+--+--+--+--+--+--+--+--+
fn horizontal_border() -> String {
    let mut hz_border = String::from("+");
    for _i in 0..WIDTH {
        hz_border.push_str("--+");
    }
    hz_border
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("\n{}", horizontal_border());

        for (_i, row) in self.rows.iter().rev().enumerate() {
            out.push_str("\n|");

            for cell in row {
                if *cell == 1 {
                    out.push_str("[]|");
                } else {
                    out.push_str("  |")
                }
            }
            let bottom = format!("\n{}", horizontal_border());
            out.push_str(bottom.as_str())
        }

        write!(f, "{out}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut matrix = Matrix::empty();
        assert_eq!(matrix.set(0, 0), Some(0));
        assert_eq!(matrix.set(0, 0), Some(1));
    }

    #[test]
    fn test_get() {
        let mut matrix = Matrix::empty();
        matrix.set(1, 1);
        assert_eq!(matrix.get(1, 1), Some(1));
    }

    #[test]
    fn test_accept_success() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        let mut expected = Matrix::empty();
        expected.set(4, 18);
        expected.set(5, 18);
        expected.set(5, 19);
        expected.set(6, 19);
        assert_eq!(*matrix.apply(piece).unwrap(), expected);
    }

    #[test]
    fn test_accept_out_of_bounds_x() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(8, 18);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece).is_none());
    }

    #[test]
    fn test_accept_out_of_bounds_y() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(4, 19);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece).is_none());
    }

    #[test]
    fn test_accept_collision() {
        let mut matrix = Matrix::empty();
        matrix.set(5, 18);
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece).is_none());
    }
}
