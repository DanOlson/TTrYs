use std::collections::VecDeque;
use crate::{
    scoring::RowsCleared,
    piece::{Piece, Point},
    matrix::{Matrix, Cell},
    level::Level
};

pub enum GameMode {
    AType,
    BType,
}

pub struct Stats {
    pub score: usize,
    pub rows_cleared: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self { score: 0, rows_cleared: 0 }
    }

    pub fn update(&mut self, score: usize, rows: &RowsCleared) {
        self.score += score;
        self.record_rows_cleared(rows);
    }

    fn record_rows_cleared(&mut self, rows: &RowsCleared) {
        self.rows_cleared += match rows {
            RowsCleared::Zero => 0,
            RowsCleared::One => 1,
            RowsCleared::Two => 2,
            RowsCleared::Three => 3,
            RowsCleared::Four => 4,
        }
    }
}

pub struct Game {
    pub board: Matrix<Cell>,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub stats: Stats,
    pub level: Level,
    pub levels: VecDeque<Level>,
    pub wants_to_quit: bool,
    pub paused: bool,
    pub game_over: bool,
}

impl Game {
    pub fn new(mode: GameMode) -> Self {
        let board = match mode {
            GameMode::AType => Matrix::empty(),
            GameMode::BType => Matrix::random_partial_fill()
        };
        let origin = Point::new(4, 18);
        let mut levels = Level::all();
        let level = levels.pop_front().unwrap();

        Self {
            board,
            level,
            levels,
            current_piece: Piece::random(origin),
            next_piece: Piece::random(origin),
            stats: Stats::new(),
            wants_to_quit: false,
            paused: false,
            game_over: false,
        }
    }

    pub fn quit(&mut self) {
        self.wants_to_quit = true
    }

    pub fn should_quit(&self) -> bool {
        self.wants_to_quit
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
        if self.is_stopped() { return }

        if self.level.tick().is_some() {
            self.on_down();
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.game_over { return }
        self.paused = !self.paused;
    }

    fn is_stopped(&self) -> bool {
        self.paused || self.game_over
    }

    fn handle_movement<F>(&mut self, attempt_move: F) -> Option<()>
        where F: Fn(&Piece) -> Option<Piece>
    {
        if self.is_stopped() { return None }

        let projection = attempt_move(&self.current_piece)?;
        let color = self.piece_color(&projection);

        if self.board.apply(projection, color).is_some() {
            self.current_piece = projection;
            Some(())
        } else {
            None
        }
    }

    pub fn piece_color(&self, piece: &Piece) -> u8 {
        self.level.theme.piece_color(piece)
    }

    fn piece_placed(&mut self) {
        if self.is_stopped() { return }

        self.board.settle(
            &self.current_piece.points,
            self.piece_color(&self.current_piece)
        );
        let rows_cleared = self.board.clear_full_rows();
        let score = self.level.scoring_config.score(&rows_cleared);
        self.stats.update(score, &rows_cleared);
        self.update_level();
        self.current_piece = self.next_piece;
        self.next_piece = Piece::random(Point::new(4, 18));
        if !self.board.can_apply(&self.current_piece.points) {
            self.game_over = true;
        }
    }

    fn update_level(&mut self) {
        if self.stats.rows_cleared < self.level.rows_to_pass {
            return
        }

        if let Some(next_level) = self.levels.pop_front() {
            self.level = next_level;
        } else {
            self.quit()
        }
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
        matrix.apply(piece, 14).unwrap();
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
        assert_eq!(game.board.get(4, 18).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(5, 18).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(5, 19).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(6, 19).unwrap().value, Color::Gray);

        game.on_down();

        // assert new piece position
        assert_eq!(game.board.get(4, 17).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(5, 17).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(5, 18).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(6, 18).unwrap().value, Color::Gray);

        // assert previous position is unoccupied
        assert_eq!(game.board.get(4, 18).unwrap().value, Color::White);
        assert_eq!(game.board.get(5, 18).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(5, 19).unwrap().value, Color::White);
        assert_eq!(game.board.get(6, 19).unwrap().value, Color::White);
    }

    #[test]
    fn test_on_down_to_bottom() {
        let origin = Point::new(4, 0);
        let piece = Piece::hero(origin);
        let mut game = setup(piece);
        // assert initial piece position
        assert_eq!(game.board.get(4, 0).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(5, 0).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(6, 0).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(7, 0).unwrap().value, Color::Gray);

        game.on_down();

        // assert piece is settled
        assert_eq!(game.board.get(4, 0).unwrap().value, Color::Black);
        assert_eq!(game.board.get(5, 0).unwrap().value, Color::Black);
        assert_eq!(game.board.get(6, 0).unwrap().value, Color::Black);
        assert_eq!(game.board.get(7, 0).unwrap().value, Color::Black);
    }

    #[test]
    fn test_on_left() {
        let origin = Point::new(1, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert initial piece position
        assert_eq!(game.board.get(1, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(2, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(2, 2).unwrap().value, Color::Gray);

        game.on_left();

        // assert new piece position
        assert_eq!(game.board.get(0, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(0, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 2).unwrap().value, Color::Gray);
    }

    #[test]
    fn test_on_left_cannot_go_out_of_bounds() {
        let origin = Point::new(0, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert initial piece position
        assert_eq!(game.board.get(0, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(0, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 2).unwrap().value, Color::Gray);

        game.on_left();

        // assert piece position is unchanged
        assert_eq!(game.board.get(0, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(0, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(1, 2).unwrap().value, Color::Gray);
    }

    #[test]
    fn test_on_right() {
        let origin = Point::new(7, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert initial piece position
        assert_eq!(game.board.get(7, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(8, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(7, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(8, 2).unwrap().value, Color::Gray);

        game.on_right();

        // assert new piece position
        assert_eq!(game.board.get(8, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(9, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(8, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(9, 2).unwrap().value, Color::Gray);
    }

    #[test]
    fn test_on_right_cannot_go_out_of_bounds() {
        let origin = Point::new(8, 1);
        let piece = Piece::smashboy(origin);
        let mut game = setup(piece);

        // assert new piece position
        assert_eq!(game.board.get(8, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(9, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(8, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(9, 2).unwrap().value, Color::Gray);

        game.on_right();

        // assert position is unchanged
        assert_eq!(game.board.get(8, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(9, 1).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(8, 2).unwrap().value, Color::Gray);
        assert_eq!(game.board.get(9, 2).unwrap().value, Color::Gray);
    }
}
