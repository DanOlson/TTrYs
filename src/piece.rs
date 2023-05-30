use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Shape {
    OrangeRicky,
    BlueRicky,
    ClevelandZ,
    RhodeIslandZ,
    Hero,
    Teewee,
    Smashboy,
}

type PieceTransition = Box<dyn FnMut((usize, &mut Point))>;

impl Shape {
    pub fn transition(&self, from: Orientation, to: Orientation) -> PieceTransition {
        match (self, from, to) {
            // OrangeRicky
            (Shape::OrangeRicky, Orientation::One, Orientation::Two) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 1 { p.x += 1; p.y += 1 }
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::OrangeRicky, Orientation::Two, Orientation::Three) => Box::new(|(i, p)| {
                if i == 1 { p.x -= 1; p.y -= 1 }
                if i == 2 { p.x += 2; p.y -= 1 }
                if i == 3 { p.x += 1; p.y -= 2 }
            }),
            (Shape::OrangeRicky, Orientation::Three, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 2 }
                if i == 3 { p.x -= 1 }
            }),
            (Shape::OrangeRicky, Orientation::Four, Orientation::One) => Box::new(|(i, p)| {
                if i == 0 { p.x -= 1; p.y += 1 }
                if i == 1 { p.x -= 2 }
                if i == 3 { p.x += 1; p.y -= 1 }
            }),
            (Shape::OrangeRicky, Orientation::One, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.x += 2 }
                if i == 1 { p.x += 1; p.y -= 1 }
                if i == 3 { p.x -= 1; p.y += 1 }
            }),
            (Shape::OrangeRicky, Orientation::Four, Orientation::Three) => Box::new(|(i, p)| {
                if i == 0 { p.x -= 1 }
                if i == 2 { p.y -= 1 }
                if i == 3 { p.x += 1; p.y -= 1 }
            }),
            (Shape::OrangeRicky, Orientation::Three, Orientation::Two) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 2 { p.x -= 1; p.y += 1 }
                if i == 3 { p.x -= 1; p.y += 1 }
            }),
            (Shape::OrangeRicky, Orientation::Two, Orientation::One) => Box::new(|(i, p)| {
                if i == 0 { p.x -= 1; p.y += 1 }
                if i == 2 { p.y -= 2 }
                if i == 3 { p.x += 1; p.y -= 1 }
            }),

            // BlueRicky
            (Shape::BlueRicky, Orientation::One, Orientation::Two) => Box::new(|(i, p)| {
                if i == 0 { p.x -= 2 }
                if i == 1 { p.x += 1; p.y -= 1 }
                if i == 3 { p.x -= 1; p.y += 1 }
            }),
            (Shape::BlueRicky, Orientation::Two, Orientation::Three) => Box::new(|(i, p)| {
                if i == 2 { p.x += 1; p.y -= 1 }
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::BlueRicky, Orientation::Three, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 1 }
                if i == 2 { p.y += 2 }
                if i == 3 { p.x += 1; p.y += 1 }
            }),
            (Shape::BlueRicky, Orientation::Four, Orientation::One) => Box::new(|(i, p)| {
                if i == 0 { p.x -= 1; p.y += 1 }
                if i == 2 { p.x += 1; p.y -= 1 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::BlueRicky, Orientation::One, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 1 { p.x += 1; p.y -= 1 }
                if i == 3 { p.x -= 1; p.y += 1 }
            }),
            (Shape::BlueRicky, Orientation::Four, Orientation::Three) => Box::new(|(i, p)| {
                if i == 1 { p.x -= 1; p.y -= 1 }
                if i == 2 { p.x -= 1; p.y -= 1 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::BlueRicky, Orientation::Three, Orientation::Two) => Box::new(|(i, p)| {
                if i == 2 { p.x -= 1; p.y += 1 }
                if i == 3 { p.x += 1; p.y += 1 }
            }),
            (Shape::BlueRicky, Orientation::Two, Orientation::One) => Box::new(|(i, p)| {
                if i == 0 { p.y += 1 }
                if i == 1 { p.x += 1 }
                if i == 3 { p.x += 1; p.y -= 1 }
            }),

            // Cleveland Z
            (Shape::ClevelandZ, Orientation::One, Orientation::Two) => Box::new(|(i, p)| {
                if i == 1 { p.y += 2 }
                if i == 2 { p.x += 2 }
            }),
            (Shape::ClevelandZ, Orientation::Two, Orientation::Three) => Box::new(|(i, p)| {
                if i == 2 { p.x -= 2 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::ClevelandZ, Orientation::Three, Orientation::Four) => Box::new(|(i, p)| {
                if i == 1 { p.y += 2 }
                if i == 2 { p.x += 2 }
            }),
            (Shape::ClevelandZ, Orientation::Four, Orientation::One) => Box::new(|(i, p)| {
                if i == 2 { p.x -= 2 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::ClevelandZ, Orientation::One, Orientation::Four) => Box::new(|(i, p)| {
                if i == 1 { p.y += 2 }
                if i == 2 { p.x += 2 }
            }),
            (Shape::ClevelandZ, Orientation::Four, Orientation::Three) => Box::new(|(i, p)| {
                if i == 2 { p.x -= 2 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::ClevelandZ, Orientation::Three, Orientation::Two) => Box::new(|(i, p)| {
                if i == 1 { p.y += 2 }
                if i == 2 { p.x += 2 }
            }),
            (Shape::ClevelandZ, Orientation::Two, Orientation::One) => Box::new(|(i, p)| {
                if i == 2 { p.x -= 2 }
                if i == 3 { p.y -= 2 }
            }),

            // RhodeIslandZ
            (Shape::RhodeIslandZ, Orientation::One, Orientation::Two) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 3 { p.x -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::Two, Orientation::Three) => Box::new(|(i, p)| {
                if i == 1 { p.x += 2 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::Three, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 3 { p.x -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::Four, Orientation::One) => Box::new(|(i, p)| {
                if i == 1 { p.x += 2 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::One, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 3 { p.x -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::Four, Orientation::Three) => Box::new(|(i, p)| {
                if i == 1 { p.x += 2 }
                if i == 3 { p.y -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::Three, Orientation::Two) => Box::new(|(i, p)| {
                if i == 0 { p.y += 2 }
                if i == 3 { p.x -= 2 }
            }),
            (Shape::RhodeIslandZ, Orientation::Two, Orientation::One) => Box::new(|(i, p)| {
                if i == 1 { p.x += 2 }
                if i == 3 { p.y -= 2 }
            }),

            // Hero
            (Shape::Hero, Orientation::One, Orientation::Two) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 2; p.y += 2 }
                if i == 1 { p.x += 1; p.y += 1 }
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::Hero, Orientation::Two, Orientation::Three) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 1 }
                if i == 2 { p.x -= 1; p.y -= 1 }
                if i == 3 { p.x -= 2; p.y -= 2 }
            }),
            (Shape::Hero, Orientation::Three, Orientation::Four) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 2; p.y += 2 }
                if i == 1 { p.x += 1; p.y += 1 }
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::Hero, Orientation::Four, Orientation::One) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 1 }
                if i == 2 { p.x -= 1; p.y -= 1 }
                if i == 3 { p.x -= 2; p.y -= 2 }
            }),
            (Shape::Hero, Orientation::One, Orientation::Four) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 2; p.y += 2 }
                if i == 1 { p.x += 1; p.y += 1 }
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::Hero, Orientation::Four, Orientation::Three) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 1 }
                if i == 2 { p.x -= 1; p.y -= 1 }
                if i == 3 { p.x -= 2; p.y -= 2 }
            }),
            (Shape::Hero, Orientation::Three, Orientation::Two) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 2; p.y += 2 }
                if i == 1 { p.x += 1; p.y += 1 }
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::Hero, Orientation::Two, Orientation::One) =>  Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 1 }
                if i == 2 { p.x -= 1; p.y -= 1 }
                if i == 3 { p.x -= 2; p.y -= 2 }
            }),

            // Teewee
            (Shape::Teewee, Orientation::One, Orientation::Two) => Box::new(|(i, p)| {
                if i == 3 { p.x -= 1; p.y += 1 }
            }),
            (Shape::Teewee, Orientation::Two, Orientation::Three) => Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y += 1 }
            }),
            (Shape::Teewee, Orientation::Three, Orientation::Four) => Box::new(|(i, p)| {
                if i == 0 { p.x += 1; p.y -= 1 }
            }),
            (Shape::Teewee, Orientation::Four, Orientation::One) => Box::new(|(i, p)| {
                if i == 3 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::Teewee, Orientation::One, Orientation::Four) => Box::new(|(i, p)| {
                if i == 1 { p.x += 1; p.y += 1 }
            }),
            (Shape::Teewee, Orientation::Four, Orientation::Three) => Box::new(|(i, p)| {
                if i == 0 { p.x -= 1; p.y += 1 }
            }),
            (Shape::Teewee, Orientation::Three, Orientation::Two) => Box::new(|(i, p)| {
                if i == 2 { p.x -= 1; p.y -= 1 }
            }),
            (Shape::Teewee, Orientation::Two, Orientation::One) => Box::new(|(i, p)| {
                if i == 3 { p.x += 1; p.y -= 1 }
            }),

            // Smashboy
            (Shape::Smashboy, _, _) => Box::new(|(_i, _p)| {}),

            _ => panic!()
        }

    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Orientation {
    One,
    Two,
    Three,
    Four,
}

impl Orientation {
    pub fn next(&self) -> Self {
        match self {
            Orientation::One => Orientation::Two,
            Orientation::Two => Orientation::Three,
            Orientation::Three => Orientation::Four,
            Orientation::Four => Orientation::One,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Orientation::One => Orientation::Four,
            Orientation::Two => Orientation::One,
            Orientation::Three => Orientation::Two,
            Orientation::Four => Orientation::Three,
        }
    }
}

pub struct Piece {
    pub shape: Shape,
    pub orientation: Orientation,
    pub points: [Point; 4],
}

impl Piece {
    pub fn new(shape: Shape, points: [Point; 4]) -> Self {
        Self { shape, points, orientation: Orientation::One }
    }

    //
    // [][][]
    // []
    //
    pub fn orange_ricky(origin: Point) -> Self {
        let points = [
            Point::new(origin.x, origin.y),
            Point::new(origin.x, origin.y + 1),
            Point::new(origin.x + 1, origin.y + 1),
            Point::new(origin.x + 2, origin.y + 1),
        ];
        Self::new(Shape::OrangeRicky, points)
    }

    //
    // [][][]
    //     []
    //
    pub fn blue_ricky(origin: Point) -> Self {
        let points = [
            Point::new(origin.x + 2, origin.y),
            Point::new(origin.x, origin.y + 1),
            Point::new(origin.x + 1, origin.y + 1),
            Point::new(origin.x + 2, origin.y + 1),
        ];
        Self::new(Shape::BlueRicky, points)
    }

    //
    // [][]
    //   [][]
    //
    pub fn cleveland_z(origin: Point) -> Self {
        let points = [
            Point::new(origin.x + 1, origin.y),
            Point::new(origin.x + 2, origin.y),
            Point::new(origin.x, origin.y + 1),
            Point::new(origin.x + 1, origin.y + 1),
        ];
        Self::new(Shape::ClevelandZ, points)
    }

    //
    //   [][]
    // [][]
    //
    pub fn rhode_island_z(origin: Point) -> Self {
        let points = [
            Point::new(origin.x, origin.y),
            Point::new(origin.x + 1, origin.y),
            Point::new(origin.x + 1, origin.y + 1),
            Point::new(origin.x + 2, origin.y + 1),
        ];
        Self::new(Shape::RhodeIslandZ, points)
    }

    //
    // [][][][]
    //
    pub fn hero(origin: Point) -> Self {
        let points = [
            Point::new(origin.x, origin.y),
            Point::new(origin.x + 1, origin.y),
            Point::new(origin.x + 2, origin.y),
            Point::new(origin.x + 3, origin.y),
        ];
        Self::new(Shape::Hero, points)
    }

    //
    // [][][]
    //   []
    //
    pub fn teewee(origin: Point) -> Self {
        let points = [
            Point::new(origin.x + 1, origin.y),
            Point::new(origin.x, origin.y + 1),
            Point::new(origin.x + 1, origin.y + 1),
            Point::new(origin.x + 2, origin.y + 1),
        ];
        Self::new(Shape::Teewee, points)
    }

    //
    // [][]
    // [][]
    //
    pub fn smashboy(origin: Point) -> Self {
        let points = [
            Point::new(origin.x, origin.y),
            Point::new(origin.x + 1, origin.y),
            Point::new(origin.x, origin.y + 1),
            Point::new(origin.x + 1, origin.y + 1),
        ];
        Self::new(Shape::Smashboy, points)
    }

    pub fn move_left(&mut self) {
        self.points
            .iter_mut()
            .for_each(|p| p.x -= 1);
    }

    pub fn move_right(&mut self) {
        self.points
            .iter_mut()
            .for_each(|p| p.x += 1)
    }

    pub fn move_down(&mut self) {
        self.points
            .iter_mut()
            .for_each(|p| p.y -= 1)
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotate(self.orientation.next());
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.rotate(self.orientation.prev());
    }

    fn rotate(&mut self, next_orientation: Orientation) {
        let transition = self.shape.transition(
            self.orientation,
            next_orientation
        );
        self.points
            .iter_mut()
            .enumerate()
            .for_each(transition);
        self.sort_points();
        self.orientation = next_orientation;
    }

    fn sort_points(&mut self) {
        self.points.sort_by(|a, b| {
            let ordering = a.y.cmp(&b.y);
            match ordering {
                Ordering::Equal => a.x.cmp(&b.x),
                _ => ordering
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_points_orange_ricky() {
        let origin = Point::new(6, 19);
        let piece = Piece::orange_ricky(origin);
        assert_eq!(piece.points, [
            Point::new(6, 19),
            Point::new(6, 20),
            Point::new(7, 20),
            Point::new(8, 20)
        ]);
    }

    #[test]
    fn test_move_left_orange_ricky() {
        let origin = Point::new(6, 19);
        let mut piece = Piece::orange_ricky(origin);
        piece.move_left();
        assert_eq!(piece.points, [
            Point::new(5, 19),
            Point::new(5, 20),
            Point::new(6, 20),
            Point::new(7, 20)
        ]);
    }

    #[test]
    fn test_move_right_orange_ricky() {
        let origin = Point::new(6, 19);
        let mut piece = Piece::orange_ricky(origin);
        piece.move_right();
        assert_eq!(piece.points, [
            Point::new(7, 19),
            Point::new(7, 20),
            Point::new(8, 20),
            Point::new(9, 20)
        ]);
    }

    #[test]
    fn test_move_down_orange_ricky() {
        let origin = Point::new(6, 19);
        let mut piece = Piece::orange_ricky(origin);
        piece.move_down();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(8, 19)
        ])
    }

    #[test]
    fn test_orange_ricky_rotate_clockwise() {
        let origin = Point::new(6, 18);
        let mut piece = Piece::orange_ricky(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(7, 18),
            Point::new(7, 19),
            Point::new(6, 20),
            Point::new(7, 20),
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18),
            Point::new(8, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(7, 18),
            Point::new(8, 18),
            Point::new(7, 19),
            Point::new(7, 20),
            ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(8, 19),
        ]);
    }

    #[test]
    fn test_orange_ricky_rotate_counterclockwise() {
        let origin = Point::new(6, 18);
        let mut piece = Piece::orange_ricky(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(7, 18),
            Point::new(8, 18),
            Point::new(7, 19),
            Point::new(7, 20),
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18),
            Point::new(8, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(7, 18),
            Point::new(7, 19),
            Point::new(6, 20),
            Point::new(7, 20),
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(8, 19),
        ]);
    }

    #[test]
    fn test_points_blue_ricky() {
        let origin = Point::new(6, 19);
        let piece = Piece::blue_ricky(origin);
        assert_eq!(piece.points, [
            Point::new(8, 19),
            Point::new(6, 20),
            Point::new(7, 20),
            Point::new(8, 20)
        ]);
    }

    #[test]
    fn test_blue_ricky_rotate_clockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::blue_ricky(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(6, 20),
            Point::new(7, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_blue_ricky_rotate_counterclockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::blue_ricky(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(6, 20),
            Point::new(7, 20),
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_points_cleveland_z() {
        let origin = Point::new(5, 18);
        let piece = Piece::cleveland_z(origin);
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
    }

    #[test]
    fn test_cleveland_z_rotate_clockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::cleveland_z(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
    }

    #[test]
    fn test_cleveland_z_rotate_counterclockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::cleveland_z(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
    }

    #[test]
    fn test_points_rhode_island_z() {
        let origin = Point::new(5, 18);
        let piece = Piece::rhode_island_z(origin);
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_rhode_island_z_rotate_clockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::rhode_island_z(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(5, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(5, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_rhode_island_z_rotate_counterclockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::rhode_island_z(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(5, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(5, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_points_hero() {
        let origin = Point::new(5, 18);
        let piece = Piece::hero(origin);
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18)
        ]);
    }

    #[test]
    fn test_hero_rotate_clockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::hero(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(7, 17),
            Point::new(7, 18),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(7, 17),
            Point::new(7, 18),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18)
        ]);
    }

    #[test]
    fn test_hero_rotate_counterclockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::hero(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(7, 17),
            Point::new(7, 18),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(7, 17),
            Point::new(7, 18),
            Point::new(7, 19),
            Point::new(7, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(7, 18),
            Point::new(8, 18)
        ]);
    }

    #[test]
    fn test_points_teewee() {
        let origin = Point::new(5, 18);
        let piece = Piece::teewee(origin);
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19),
        ]);
    }

    #[test]
    fn test_teewee_rotate_clockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::teewee(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_teewee_rotate_counterclockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::teewee(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(6, 20)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19),
            Point::new(7, 19)
        ]);
    }

    #[test]
    fn test_points_smashboy() {
        let origin = Point::new(5, 18);
        let piece = Piece::smashboy(origin);
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
    }

    #[test]
    fn test_smashboy_rotate_clockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::smashboy(origin);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_clockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
    }

    #[test]
    fn test_smashboy_rotate_counterclockwise() {
        let origin = Point::new(5, 18);
        let mut piece = Piece::smashboy(origin);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
        piece.rotate_counterclockwise();
        assert_eq!(piece.points, [
            Point::new(5, 18),
            Point::new(6, 18),
            Point::new(5, 19),
            Point::new(6, 19)
        ]);
    }
}
