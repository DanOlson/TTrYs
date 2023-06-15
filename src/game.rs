use crate::piece::{Piece, Point};
use crate::matrix::Matrix;

pub enum GameMode {
    AType,
    BType,
}

pub struct Game {
    pub board: Matrix,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub score: usize,
}

impl Game {
    pub fn new(mode: GameMode) -> Self {
        let board = match mode {
            GameMode::AType => Matrix::empty(),
            GameMode::BType => Matrix::random_partial_fill()
        };
        let origin = Point::new(4, 18);
        Self {
            board,
            current_piece: Piece::random(origin),
            next_piece: Piece::random(origin),
            score: 0,
        }
    }

    pub fn on_left(&mut self) {
        self.handle_movement(Piece::move_left);
    }

    pub fn on_right(&mut self) {
        self.handle_movement(Piece::move_right);
    }

    pub fn on_down(&mut self) {
        if self.handle_movement(Piece::move_down).is_some() {
            self.piece_placed();
        }
    }
    pub fn on_rotate_clockwise(&mut self) {
        self.handle_movement(Piece::rotate_clockwise);
    }
    pub fn on_rotate_counterclockwise(&mut self) {
        self.handle_movement(Piece::rotate_counterclockwise);
    }

    pub fn on_tick(&mut self) {}

    fn handle_movement(&mut self, move_piece: impl Fn(&mut Piece)) -> Option<()> {
        let prev_piece = self.current_piece;
        move_piece(&mut self.current_piece);
        if self.board.apply(self.current_piece).is_some() {
            Some(())
        } else {
            self.current_piece = prev_piece;
            None
        }
    }

    // Handle the placement of a piece. Clear rows, update
    // score, etc.
    fn piece_placed(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        matrix::{Matrix, Color},
        piece::{Piece, Point}
    };

    fn setup() -> Game {
        let mut game = Game::new(GameMode::AType);
        let mut matrix = Matrix::empty();
        let origin = Point::new(4, 18);
        let piece = Piece::rhode_island_z(origin);
        matrix.apply(piece).unwrap();
        game.current_piece = piece;
        game.board = matrix;
        game
    }

    #[test]
    fn test_on_down() {
        let mut game = setup();
        // assert initial piece position
        assert_eq!(game.board.get(4, 18), Some(Color::Gray));
        assert_eq!(game.board.get(5, 18), Some(Color::Gray));
        assert_eq!(game.board.get(5, 19), Some(Color::Gray));
        assert_eq!(game.board.get(6, 19), Some(Color::Gray));

        println!("{:?}", game.board);
        game.on_down();
        println!("{:?}", game.board);

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
}
