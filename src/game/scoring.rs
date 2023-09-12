pub enum RowsCleared {
    Zero,
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone, Copy)]
pub struct ScoringConfig {
    pub one: usize,
    pub two: usize,
    pub three: usize,
    pub four: usize,
}

impl ScoringConfig {
    pub fn new(one: usize, two: usize, three: usize, four: usize) -> Self {
        Self { one, two, three, four }
    }

    pub fn score(&self, rows_cleared: &RowsCleared) -> usize {
        match rows_cleared {
            RowsCleared::Zero => 0,
            RowsCleared::One => self.one,
            RowsCleared::Two => self.two,
            RowsCleared::Three => self.three,
            RowsCleared::Four => self.four,
        }
    }
}
