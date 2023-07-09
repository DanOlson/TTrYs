use rand::{thread_rng, Rng};
use crate::scoring::RowsCleared;
use crate::piece::{Piece, Point};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const EMPTY_ROW: [Color; WIDTH] = [Color::White; WIDTH];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
    Gray,
}

#[derive(PartialEq)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>,
    // pub rows: [[T; WIDTH]; HEIGHT]
}

impl <T: Copy> Matrix<T> {
    pub fn new(width: usize, height: usize, initial_value: T) -> Self
        where T: Copy
    {
        let row = vec![initial_value; width];
        let rows = vec![row; height];
        Self { rows }
    }

    pub fn square(length: usize, initial_value: T) -> Self
        where T: Copy
    {
        Self::new(length, length, initial_value)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        Some(*self.rows.get(y)?.get(x)?)
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) -> Option<T> {
        let mut row = self.rows.get(y)?.to_owned();
        let prev = *row.get(x)?;
        row[x] = val;
        self.rows[y] = row.to_vec();
        Some(prev)
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

impl Matrix<Color> {
    pub fn empty() -> Self {
        Matrix::new(WIDTH, HEIGHT, Color::White)
    }

    pub fn random_partial_fill() -> Self {
        let mut rows = vec![Vec::from(EMPTY_ROW); HEIGHT];
        (0..HEIGHT).for_each(|i| {
            if i < HEIGHT - 15 {
                rows[i] = Self::random_row();
            }
        });

        Matrix { rows }
    }

    fn random_row() -> Vec<Color> {
        let mut row = vec![Color::White; WIDTH];
        row.iter_mut().take(WIDTH).for_each(|i| {
            if thread_rng().gen_bool(1.0 / 3.0) { *i = Color::Black }
        });
        row
    }

    fn unset(&mut self, x: usize, y: usize) {
        let mut row = self.rows.get(y).unwrap().to_owned();
        row[x] = Color::White;
        self.rows[y] = row;
    }

    pub fn apply(&mut self, piece: Piece) -> Option<&Matrix<Color>> {
        if !self.can_apply(&piece.points) { return None }

        let greys = self.rows
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (y, row)| {
                row.iter().enumerate().for_each(|(x, color)| {
                    if color == &Color::Gray {
                        acc.push(Point::new(x, y));
                    }
                });
                acc
            });
        greys.iter().for_each(|p| { self.set(p.x, p.y, Color::White); });

        piece.points
            .iter()
            .for_each(|p| { self.set(p.x, p.y, Color::Gray); });

        Some(self)
    }

    pub fn settle(&mut self, points: &[Point]) -> Option<&Matrix<Color>> {
        if !self.can_apply(points) { return None }

        points.iter()
            .for_each(|p| { self.set(p.x, p.y, Color::Black); });
        Some(self)
    }

    pub fn clear_full_rows(&mut self) -> RowsCleared {
        let full_indices = self.rows
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                if row.iter().all(|c| c.eq(&Color::Black)) {
                    Some(y)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let points_to_clear = self.rows
            .iter()
            .enumerate()
            .fold(vec![], |mut acc: Vec<Point>, (y, row)| {
                if !full_indices.contains(&y) { return acc }

                row.iter()
                    .enumerate()
                    .for_each(|(x, _)| {
                        acc.push(Point::new(x, y));
                    });
                acc
            });
        points_to_clear
            .iter()
            .for_each(|p| self.unset(p.x, p.y));
        let points_to_drop = self.rows
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (y, row)| {
                if full_indices.contains(&y) { return acc }
                row.iter()
                    .enumerate()
                    .for_each(|(x, color)| {
                        if color == &Color::Black {
                            acc.push(Point::new(x, y));
                        }
                    });
                acc
            });
        points_to_drop
            .iter()
            .for_each(|p| self.unset(p.x, p.y));

        points_to_drop
            .iter()
            .for_each(|p| {
                let drop_by = full_indices
                    .iter()
                    .filter(|y| p.y > **y)
                    .count();
                self.set(p.x, p.y - drop_by, Color::Black);
            });

        match full_indices.len() {
            1 => RowsCleared::One,
            2 => RowsCleared::Two,
            3 => RowsCleared::Three,
            4 => RowsCleared::Four,
            _ => RowsCleared::Zero
        }
    }

    fn can_apply(&self, points: &[Point]) -> bool {
        points
            .iter()
            .all(|p| self.get(p.x, p.y).ne(&Some(Color::Black)))
    }
}

// +--+--+--+--+--+--+--+--+--+--+--+
fn horizontal_border() -> String {
    let mut hz_border = String::from(".");
    for _i in 0..WIDTH {
        hz_border.push_str("  .");
    }
    hz_border
}

impl std::fmt::Debug for Matrix<Color> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for Matrix<Color> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("\r\n{}", horizontal_border());

        self.rows.iter().rev().for_each(|row| {
            out.push_str("\r\n ");
            row.iter().for_each(|cell| {
                if *cell == Color::White {
                    out.push_str("   ")
                } else {
                    out.push_str("[] ");
                }
            });
            let bottom = format!("\r\n{}", horizontal_border());
            out.push_str(bottom.as_str());
        });

        write!(f, "{out}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Point;

    #[test]
    fn test_set() {
        let mut matrix = Matrix::empty();
        assert_eq!(matrix.set(0, 0, Color::Black), Some(Color::White));
        assert_eq!(matrix.set(0, 0, Color::Black), Some(Color::Black));
    }

    #[test]
    fn test_get() {
        let mut matrix = Matrix::empty();
        matrix.set(1, 1, Color::Black);
        assert_eq!(matrix.get(1, 1), Some(Color::Black));
    }

    #[test]
    fn test_accept_success() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        let mut expected = Matrix::empty();
        expected.set(4, 18, Color::Gray);
        expected.set(5, 18, Color::Gray);
        expected.set(5, 19, Color::Gray);
        expected.set(6, 19, Color::Gray);
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
        matrix.set(5, 18, Color::Black);
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece).is_none());
    }
}
