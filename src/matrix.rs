use rand::{thread_rng, Rng};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const EMPTY_ROW: [u8; WIDTH] = [0; WIDTH];
pub struct Matrix {
    pub rows: [[u8; WIDTH]; HEIGHT]
}

impl Matrix {
    pub fn a_type() -> Self {
        Matrix {
            rows: [EMPTY_ROW; HEIGHT]
        }
    }

    pub fn b_type() -> Self {
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

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = horizontal_border();

        for (_i, row) in self.rows.iter().enumerate() {
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
        let mut matrix = Matrix::a_type();
        assert_eq!(matrix.set(0, 0), Some(0));
        assert_eq!(matrix.set(0, 0), Some(1));
    }

    #[test]
    fn test_get() {
        let mut matrix = Matrix::a_type();
        matrix.set(1, 1);
        assert_eq!(matrix.get(1, 1), Some(1));
    }
}
