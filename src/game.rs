use crate::piece::{Piece, Point};
use crate::matrix::{Matrix, Color};

pub enum GameMode {
    AType,
    BType,
}

#[derive(Copy, Clone)]
pub struct Level {
    pub ticks_per_drop: usize,
    pub counter: usize,
}

impl Level {
    pub fn new(ticks_per_drop: usize) -> Self {
        Self {
            ticks_per_drop,
            counter: 0
        }
    }

    pub fn tick(&mut self) -> Option<()> {
        self.counter += 1;
        if self.counter < self.ticks_per_drop {
            None
        } else {
            self.counter = 0;
            Some(())
        }
    }
}

pub struct Stats {
    pub score: usize,
}

pub struct Game {
    pub board: Matrix<Color>,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub stats: Stats,
    pub level: Level,
    pub levels: Vec<Level>
}

impl Game {
    pub fn new(mode: GameMode) -> Self {
        let board = match mode {
            GameMode::AType => Matrix::empty(),
            GameMode::BType => Matrix::random_partial_fill()
        };
        let origin = Point::new(4, 18);
        let mut levels = Game::all_levels();
        let level = levels.pop().unwrap().to_owned();

        Self {
            board,
            level,
            levels,
            current_piece: Piece::random(origin),
            next_piece: Piece::random(origin),
            stats: Stats { score: 0 },
        }
    }

    fn all_levels() -> Vec<Level> {
        vec![
            Level::new(40),
            Level::new(44),
            Level::new(48),
            Level::new(52),
            Level::new(56),
            Level::new(60),
        ]
    }

    pub fn on_left(&mut self) {
        self.handle_movement(Piece::project_left);
    }

    pub fn on_right(&mut self) {
        self.handle_movement(Piece::project_right);
    }

    pub fn on_down(&mut self) {
        if self.handle_movement(Piece::project_down).is_none() {
            self.piece_placed();
        }
    }

    pub fn on_rotate_clockwise(&mut self) {
        self.handle_movement(Piece::project_clockwise_rotation);
    }

    pub fn on_rotate_counterclockwise(&mut self) {
        self.handle_movement(Piece::project_counterclockwise_rotation);
    }

    pub fn on_tick(&mut self) {
        if self.level.tick().is_some() {
            self.on_down();
        }
    }

    fn handle_movement<F>(&mut self, attempt_move: F) -> Option<()>
        where F: Fn(&Piece) -> Option<Piece>
    {
        let projection = attempt_move(&self.current_piece)?;

        if self.board.apply(projection).is_some() {
            self.current_piece = projection;
            Some(())
        } else {
            None
        }
    }

    // Handle the placement of a piece.
    //  Clear rows (todo)
    //  update score (todo)
    //  next piece becomes current
    //  select new next piece
    fn piece_placed(&mut self) {
        self.board.settle(&self.current_piece.points);
        self.board.clear_full_rows();
        self.current_piece = self.next_piece;
        self.next_piece = Piece::random(Point::new(4, 18));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        matrix::{Matrix, Color},
        piece::{Piece, Point}
    };

    fn setup(piece: Piece) -> Game {
        let mut game = Game::new(GameMode::AType);
        let mut matrix = Matrix::empty();
        matrix.apply(piece).unwrap();
        game.current_piece = piece;
        game.board = matrix;
        game
    }

    #[test]
    fn test_on_down() {
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        let mut game = setup(piece);
        // assert initial piece position
        assert_eq!(game.board.get(4, 18), Some(Color::Gray));
        assert_eq!(game.board.get(5, 18), Some(Color::Gray));
        assert_eq!(game.board.get(5, 19), Some(Color::Gray));
        assert_eq!(game.board.get(6, 19), Some(Color::Gray));

        game.on_down();

        // assert new piece position
        assert_eq!(game.board.get(4, 17), Some(Color::Gray));
        assert_eq!(game.board.get(5, 17), Some(Color::Gray));
        assert_eq!(game.board.get(5, 18), Some(Color::Gray));
        assert_eq!(game.board.get(6, 18), Some(Color::Gray));

        // assert previous position is unoccupied
        assert_eq!(game.board.get(4, 18), Some(Color::White));
        assert_eq!(game.board.get(5, 18), Some(Color::Gray));
        assert_eq!(game.board.get(5, 19), Some(Color::White));
        assert_eq!(game.board.get(6, 19), Some(Color::White));
    }

    #[test]
    fn test_on_down_to_bottom() {
        let origin = Point::new(4, 0);
        let piece = Piece::hero(origin);
        let mut game = setup(piece);
        // assert initial piece position
        assert_eq!(game.board.get(4, 0), Some(Color::Gray));
        assert_eq!(game.board.get(5, 0), Some(Color::Gray));
        assert_eq!(game.board.get(6, 0), Some(Color::Gray));
        assert_eq!(game.board.get(7, 0), Some(Color::Gray));

        game.on_down();

        // assert piece is settled
        assert_eq!(game.board.get(4, 0), Some(Color::Black));
        assert_eq!(game.board.get(5, 0), Some(Color::Black));
        assert_eq!(game.board.get(6, 0), Some(Color::Black));
        assert_eq!(game.board.get(7, 0), Some(Color::Black));
    }

    #[test]
    fn test_on_left() {
        let origin = Point::new(1, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert initial piece position
        assert_eq!(game.board.get(1, 1), Some(Color::Gray));
        assert_eq!(game.board.get(2, 1), Some(Color::Gray));
        assert_eq!(game.board.get(1, 2), Some(Color::Gray));
        assert_eq!(game.board.get(2, 2), Some(Color::Gray));

        game.on_left();

        // assert new piece position
        assert_eq!(game.board.get(0, 1), Some(Color::Gray));
        assert_eq!(game.board.get(1, 1), Some(Color::Gray));
        assert_eq!(game.board.get(0, 2), Some(Color::Gray));
        assert_eq!(game.board.get(1, 2), Some(Color::Gray));
    }

    #[test]
    fn test_on_left_cannot_go_out_of_bounds() {
        let origin = Point::new(0, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert initial piece position
        assert_eq!(game.board.get(0, 1), Some(Color::Gray));
        assert_eq!(game.board.get(1, 1), Some(Color::Gray));
        assert_eq!(game.board.get(0, 2), Some(Color::Gray));
        assert_eq!(game.board.get(1, 2), Some(Color::Gray));

        game.on_left();

        // assert piece position is unchanged
        assert_eq!(game.board.get(0, 1), Some(Color::Gray));
        assert_eq!(game.board.get(1, 1), Some(Color::Gray));
        assert_eq!(game.board.get(0, 2), Some(Color::Gray));
        assert_eq!(game.board.get(1, 2), Some(Color::Gray));
    }

    #[test]
    fn test_on_right() {
        let origin = Point::new(7, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert initial piece position
        assert_eq!(game.board.get(7, 1), Some(Color::Gray));
        assert_eq!(game.board.get(8, 1), Some(Color::Gray));
        assert_eq!(game.board.get(7, 2), Some(Color::Gray));
        assert_eq!(game.board.get(8, 2), Some(Color::Gray));

        game.on_right();

        // assert new piece position
        assert_eq!(game.board.get(8, 1), Some(Color::Gray));
        assert_eq!(game.board.get(9, 1), Some(Color::Gray));
        assert_eq!(game.board.get(8, 2), Some(Color::Gray));
        assert_eq!(game.board.get(9, 2), Some(Color::Gray));
    }

    #[test]
    fn test_on_right_cannot_go_out_of_bounds() {
        let origin = Point::new(8, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert new piece position
        assert_eq!(game.board.get(8, 1), Some(Color::Gray));
        assert_eq!(game.board.get(9, 1), Some(Color::Gray));
        assert_eq!(game.board.get(8, 2), Some(Color::Gray));
        assert_eq!(game.board.get(9, 2), Some(Color::Gray));

        game.on_right();

        // assert position is unchanged
        assert_eq!(game.board.get(8, 1), Some(Color::Gray));
        assert_eq!(game.board.get(9, 1), Some(Color::Gray));
        assert_eq!(game.board.get(8, 2), Some(Color::Gray));
        assert_eq!(game.board.get(9, 2), Some(Color::Gray));
    }
}
