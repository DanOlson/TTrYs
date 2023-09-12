use rand::{thread_rng, Rng};
use super::scoring::RowsCleared;
use super::piece::{Piece, Point};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
    Gray,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub value: Color,
    pub color: u8,
}

impl Cell {
    pub fn black(color: u8) -> Self {
        Self {
            value: Color::Black,
            color
        }
    }

    pub fn gray(color: u8) -> Self {
        Self {
            value: Color::Gray,
            color
        }
    }

    pub fn white() -> Self {
        Self {
            value: Color::White,
            color: 15
        }
    }
}

#[derive(PartialEq)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>,
}

impl <T: Copy> Matrix<T> {
    pub fn new(width: usize, height: usize, initial_value: T) -> Self
        where T: Copy
    {
        let row = vec![initial_value; width];
        let rows = vec![row; height];
        Self { rows }
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

impl Matrix<Cell> {
    pub fn empty() -> Self {
        Matrix::new(WIDTH, HEIGHT, Cell::white())
    }

    pub fn random_partial_fill() -> Self {
        let empty_row = vec![Cell::white(); WIDTH];
        let mut rows = vec![empty_row; HEIGHT];
        (0..HEIGHT).for_each(|i| {
            if i < HEIGHT - 15 {
                rows[i] = Self::random_row();
            }
        });

        Matrix { rows }
    }

    fn random_row() -> Vec<Cell> {
        let mut row = vec![Cell::white(); WIDTH];
        row.iter_mut().take(WIDTH).for_each(|i| {
            if thread_rng().gen_bool(1.0 / 3.0) {
                // todo: set random color as well
                i.value = Color::Black
            }
        });
        row
    }

    fn unset(&mut self, x: usize, y: usize) {
        let mut row = self.rows.get(y).unwrap().to_owned();
        row[x] = Cell::white();
        self.rows[y] = row;
    }

    pub fn apply(&mut self, piece: Piece, color: u8) -> Option<&Self> {
        if !self.can_apply(&piece.points) { return None }

        let greys = self.rows
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (y, row)| {
                row.iter().enumerate().for_each(|(x, cell)| {
                    if cell.value == Color::Gray {
                        acc.push(Point::new(x, y));
                    }
                });
                acc
            });
        greys.iter()
            .for_each(|p| {
                self.set(p.x, p.y, Cell::white());
            });

        piece.points
            .iter()
            .for_each(|p| {
                let val = Cell {
                    value: Color::Gray,
                    color
                };
                self.set(p.x, p.y, val);
            });

        Some(self)
    }

    pub fn settle(&mut self, points: &[Point], color: u8) -> Option<&Self> {
        if !self.can_apply(points) { return None }

        points.iter()
            .for_each(|p| {
                let val = Cell {
                    value: Color::Black,
                    color,
                };
                self.set(p.x, p.y, val);
            });
        Some(self)
    }

    pub fn clear_full_rows(&mut self) -> RowsCleared {
        let full_indices = self.rows
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                if row.iter().all(|c| c.value == Color::Black) {
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
                    .for_each(|(x, cell)| {
                        if cell.value == Color::Black {
                            acc.push((Point::new(x, y), cell.color));
                        }
                    });
                acc
            });
        points_to_drop
            .iter()
            .for_each(|(p, _)| self.unset(p.x, p.y));

        points_to_drop
            .iter()
            .for_each(|(p, color)| {
                let drop_by = full_indices
                    .iter()
                    .filter(|y| p.y > **y)
                    .count();
                let val = Cell::black(*color);
                self.set(p.x, p.y - drop_by, val);
            });

        match full_indices.len() {
            1 => RowsCleared::One,
            2 => RowsCleared::Two,
            3 => RowsCleared::Three,
            4 => RowsCleared::Four,
            _ => RowsCleared::Zero
        }
    }

    pub fn can_apply(&self, points: &[Point]) -> bool {
        points
            .iter()
            .all(|p| {
                if let Some(cell) = self.get(p.x, p.y) {
                    cell.value != Color::Black
                } else {
                    false
                }
            })
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

impl std::fmt::Debug for Matrix<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for Matrix<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("\r\n{}", horizontal_border());

        self.rows.iter().rev().for_each(|row| {
            out.push_str("\r\n ");
            row.iter().for_each(|cell| {
                if cell.value == Color::White {
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

    #[test]
    fn test_set() {
        let mut matrix = Matrix::empty();
        let r1 = matrix.set(0, 0, Cell::black(1)).unwrap();
        let r2 = matrix.set(0, 0, Cell::black(1)).unwrap();
        assert_eq!(r1.value, Color::White);
        assert_eq!(r2.value, Color::Black);
    }

    #[test]
    fn test_get() {
        let mut matrix = Matrix::empty();
        matrix.set(1, 1, Cell::black(1));
        assert_eq!(matrix.get(1, 1).unwrap().value, Color::Black);
    }

    #[test]
    fn test_apply_out_of_bounds() {
        let mut matrix = Matrix::empty();
        let piece = Piece::hero(Point::new(7, 0));
        assert!(matrix.apply(piece, 1).is_none());
    }

    #[test]
    fn test_apply_success() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        let mut expected = Matrix::empty();
        expected.set(4, 18, Cell::gray(1));
        expected.set(5, 18, Cell::gray(1));
        expected.set(5, 19, Cell::gray(1));
        expected.set(6, 19, Cell::gray(1));
        assert_eq!(*matrix.apply(piece, 1).unwrap(), expected);
    }

    #[test]
    fn test_apply_out_of_bounds_x() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(8, 18);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece, 1).is_none());
    }

    #[test]
    fn test_accept_out_of_bounds_y() {
        let mut matrix = Matrix::empty();
        let origin = Point::new(4, 19);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece, 1).is_none());
    }

    #[test]
    fn test_accept_collision() {
        let mut matrix = Matrix::empty();
        matrix.set(5, 18, Cell::black(1));
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        assert!(matrix.apply(piece, 1).is_none());
    }
}
