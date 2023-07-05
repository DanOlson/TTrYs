use rand::{prelude::thread_rng, seq::SliceRandom};
use crate::rotate;

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

#[derive(Copy, Clone)]
pub struct Piece {
    pub shape: Shape,
    pub orientation: Orientation,
    pub points: [Point; 4],
}

impl Piece {
    pub fn new(shape: Shape, points: [Point; 4]) -> Self {
        Self { shape, points, orientation: Orientation::One }
    }

    pub fn random(origin: Point) -> Self {
        let shapes = [
            Shape::OrangeRicky,
            Shape::BlueRicky,
            Shape::ClevelandZ,
            Shape::RhodeIslandZ,
            Shape::Hero,
            Shape::Teewee,
            Shape::Smashboy,
        ];
        let shape = shapes.choose(&mut thread_rng())
            .copied()
            .unwrap();
        match shape {
            Shape::OrangeRicky => Self::orange_ricky(origin),
            Shape::BlueRicky => Self::blue_ricky(origin),
            Shape::ClevelandZ => Self::cleveland_z(origin),
            Shape::RhodeIslandZ => Self::rhode_island_z(origin),
            Shape::Hero => Self::hero(origin),
            Shape::Teewee => Self::teewee(origin),
            Shape::Smashboy => Self::smashboy(origin)
        }
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

    pub fn project_left(&self) -> Option<Piece> {
        let points = self.map_points(|p| {
            let point = Point::new(p.x.checked_sub(1)?, p.y);
            Some(point)
        })?;
        Some(Piece {
            points,
            shape: self.shape,
            orientation: self.orientation
        })
    }

    pub fn project_right(&self) -> Option<Piece> {
        let points = self.map_points(|p| {
            let new_x = p.x + 1;
            // todo: remove hard-coded knowledge of board width
            if new_x > 9 { return None }
            Some(Point::new(new_x, p.y))
        })?;
        Some(Piece {
            points,
            shape: self.shape,
            orientation: self.orientation
        })
    }

    pub fn project_down(&self) -> Option<Piece> {
        let points = self.map_points(|p| {
            let point = Point::new(p.x, p.y.checked_sub(1)?);
            Some(point)
        })?;
        Some(Piece {
            points,
            shape: self.shape,
            orientation: self.orientation
        })
    }

    pub fn project_clockwise_rotation(&self) -> Option<Piece> {
        rotate::rotate_clockwise(self)
    }

    pub fn project_counterclockwise_rotation(&self) -> Option<Piece> {
        rotate::rotate_counterclockwise(self)
    }

    // Return a tuple of points representing the lower left
    // and upper right points of the piece.
    pub fn bounds(&self) -> (Point, Point) {
        let min_x = self
            .points
            .iter()
            .min_by_key(|p| p.x)
            .unwrap().x;
        let max_x = self
            .points
            .iter()
            .max_by_key(|p| p.x)
            .unwrap().x;
        let min_y = self
            .points
            .iter()
            .min_by_key(|p| p.y)
            .unwrap().y;
        let max_y = self
            .points
            .iter()
            .max_by_key(|p| p.y)
            .unwrap().y;

        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    fn map_points<F>(&self, f: F) -> Option<[Point; 4]>
        where F: Fn(&Point) -> Option<Point>
    {
        let mut pts: Vec<Point> = vec![];
        for p in self.points.iter() {
            pts.push(f(p)?)
        }
        let points: [Point; 4] = pts
            .as_slice()
            .try_into()
            .unwrap();
        Some(points)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bounds_orange_ricky() {
        let origin = Point::new(3, 2);
        let piece = Piece::orange_ricky(origin);
        let (lower_left, upper_right) = piece.bounds();

        assert_eq!(lower_left, Point::new(3, 2));
        assert_eq!(upper_right, Point::new(5, 3));
    }

    #[test]
    fn test_bounds_blue_ricky() {
        let origin = Point::new(1, 1);
        let piece = Piece::blue_ricky(origin);
        let (lower_left, upper_right) = piece.bounds();

        assert_eq!(lower_left, Point::new(1, 1));
        assert_eq!(upper_right, Point::new(3, 2));
    }

    #[test]
    fn test_bounds_cleveland_z() {
        let origin = Point::new(1, 1);
        let piece = Piece::cleveland_z(origin);
        let (lower_left, upper_right) = piece.bounds();

        assert_eq!(lower_left, Point::new(1, 1));
        assert_eq!(upper_right, Point::new(3, 2));
    }

    #[test]
    fn test_bounds_rhode_island_z() {
        let origin = Point::new(1, 1);
        let piece = Piece::rhode_island_z(origin);
        let (lower_left, upper_right) = piece.bounds();

        assert_eq!(lower_left, Point::new(1, 1));
        assert_eq!(upper_right, Point::new(3, 2));
    }

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
        let piece = Piece::orange_ricky(origin).project_left().unwrap();
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
        let piece = Piece::orange_ricky(origin).project_right().unwrap();
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
        let piece = Piece::orange_ricky(origin).project_down().unwrap();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(8, 19)
        ]);
    }

    #[test]
    fn test_move_down_from_zero_orange_ricky() {
        let origin = Point::new(6, 19);
        let piece = Piece::orange_ricky(origin).project_down().unwrap();
        assert_eq!(piece.points, [
            Point::new(6, 18),
            Point::new(6, 19),
            Point::new(7, 19),
            Point::new(8, 19)
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
}
