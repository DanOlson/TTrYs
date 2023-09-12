use crate::game::{
    matrix::Matrix,
    piece::{Piece, Point, Shape, Orientation}
};

// From a given piece, return a new piece containing
// the points after rotation.
pub fn rotate_clockwise(piece: &Piece) -> Option<Piece> {
    let bbox = generate_bounding_matrix(piece);
    let mut transposed = transpose(bbox);
    reverse_rows(&mut transposed);

    let (x_offset, y_offset) = derive_offsets_for_cw(piece)?;
    let points = extract_points_from_bounding_matrix(
        &transposed,
        x_offset,
        y_offset
    );

    Some(Piece {
        shape: piece.shape,
        points,
        orientation: piece.orientation.next()
    })
}

pub fn rotate_counterclockwise(piece: &Piece) -> Option<Piece> {
    let mut bbox = generate_bounding_matrix(piece);
    reverse_rows(&mut bbox);
    let transposed = transpose(bbox);

    let (x_offset, y_offset) = derive_offsets_for_ccw(piece)?;
    let points = extract_points_from_bounding_matrix(
        &transposed,
        x_offset,
        y_offset
    );

    Some(Piece {
        shape: piece.shape,
        points,
        orientation: piece.orientation.prev()
    })
}

fn reverse_rows(matrix: &mut Matrix<usize>) {
    matrix
        .rows
        .iter_mut()
        .for_each(|row| row.reverse());
}

fn generate_bounding_matrix(piece: &Piece) -> Matrix<usize> {
    let (lower_left, upper_right) = piece.bounds();
    let width = upper_right.x - lower_left.x + 1;
    let height = upper_right.y - lower_left.y + 1;
    let mut out = Matrix::new(width, height, 0);
    piece
        .points
        .iter()
        .enumerate()
        .for_each(|(i, p)| {
            let x = p.x - lower_left.x;
            let y = p.y - lower_left.y;
            out.set(x, y, i + 1);
        });
    out
}

fn transpose(matrix: Matrix<usize>) -> Matrix<usize> {
    let height = matrix.height();
    let width = matrix.width();
    let mut out = Matrix::new(height, width, 0);

    for y in 0..height {
        for x in 0..width {
            let val = matrix.get(x, y).unwrap();
            let new_x = (width - 1) - x; // return to zero-based
            let new_y = (height - 1) - y;
            out.set(new_y, new_x, val);
        }
    }
    out
}

fn extract_points_from_bounding_matrix(
    bbox: &Matrix<usize>,
    x_offset: usize,
    y_offset: usize
) -> [Point; 4] {
    let mut points: Vec<Point> = vec![];
    for (y, row) in bbox.rows.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if i > &0 {
                let new_x = x + x_offset;
                let new_y = y + y_offset;
                points.push(Point::new(new_x, new_y));
            }
        }
    }
    points
        .as_slice()
        .try_into()
        .unwrap()
}

// returns x and y offsets relative to the bounding matrix
// based on piece shape and position.
// orientation values here are the mid-transition "previous state"
// - prior to being reassigned
fn derive_offsets_for_cw(piece: &Piece) -> Option<(usize, usize)> {
    let (lower_left, _) = piece.bounds();
    let offsets = match (piece.shape, piece.orientation) {
        (Shape::OrangeRicky, Orientation::Two) => (lower_left.x, lower_left.y + 1),
        (Shape::OrangeRicky, Orientation::Three) => (lower_left.x + 1, lower_left.y.checked_sub(1)?),
        (Shape::OrangeRicky, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::BlueRicky, Orientation::Two) => (lower_left.x, lower_left.y + 1),
        (Shape::BlueRicky, Orientation::Three) => (lower_left.x + 1, lower_left.y.checked_sub(1)?),
        (Shape::BlueRicky, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::Teewee, Orientation::Two) => (lower_left.x, lower_left.y + 1),
        (Shape::Teewee, Orientation::Three) => (lower_left.x + 1, lower_left.y.checked_sub(1)?),
        (Shape::Teewee, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::ClevelandZ, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::ClevelandZ, Orientation::Two) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::ClevelandZ, Orientation::Three) => (lower_left.x + 1, lower_left.y),
        (Shape::ClevelandZ, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::Two) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::Three) => (lower_left.x + 1, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::Hero, Orientation::One) => (lower_left.x + 2, lower_left.y.checked_sub(1)?),
        (Shape::Hero, Orientation::Two) => (lower_left.x.checked_sub(2)?, lower_left.y + 1),
        (Shape::Hero, Orientation::Three) => (lower_left.x + 2, lower_left.y.checked_sub(1)?),
        (Shape::Hero, Orientation::Four) => (lower_left.x.checked_sub(2)?, lower_left.y + 1),
        _ => (lower_left.x, lower_left.y)
    };
    Some(offsets)
}

// orientation values here are the mid-transition, "previous state"
// - prior to being reassigned
fn derive_offsets_for_ccw(piece: &Piece) -> Option<(usize, usize)> {
    let (lower_left, _) = piece.bounds();
    let offsets = match (piece.shape, piece.orientation) {
        (Shape::OrangeRicky, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::OrangeRicky, Orientation::Three) => (lower_left.x, lower_left.y.checked_sub(1)?),
        (Shape::OrangeRicky, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y + 1),
        (Shape::BlueRicky, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::BlueRicky, Orientation::Three) => (lower_left.x, lower_left.y.checked_sub(1)?),
        (Shape::BlueRicky, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y + 1),
        (Shape::Teewee, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::Teewee, Orientation::Three) => (lower_left.x, lower_left.y.checked_sub(1)?),
        (Shape::Teewee, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y + 1),
        (Shape::ClevelandZ, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::ClevelandZ, Orientation::Two) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::ClevelandZ, Orientation::Three) => (lower_left.x + 1, lower_left.y),
        (Shape::ClevelandZ, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::One) => (lower_left.x + 1, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::Two) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::Three) => (lower_left.x + 1, lower_left.y),
        (Shape::RhodeIslandZ, Orientation::Four) => (lower_left.x.checked_sub(1)?, lower_left.y),
        (Shape::Hero, Orientation::One) => (lower_left.x + 2, lower_left.y.checked_sub(1)?),
        (Shape::Hero, Orientation::Two) => (lower_left.x.checked_sub(2)?, lower_left.y + 1),
        (Shape::Hero, Orientation::Three) => (lower_left.x + 2, lower_left.y.checked_sub(1)?),
        (Shape::Hero, Orientation::Four) => (lower_left.x.checked_sub(2)?, lower_left.y + 1),
        _ => (lower_left.x, lower_left.y)
    };
    Some(offsets)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_clockwise_smashboy() {
        let origin = Point::new(6, 10);
        let piece = Piece::smashboy(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);
    }

    #[test]
    fn test_rotate_counterclockwise_smashboy() {
        let origin = Point::new(6, 10);
        let piece = Piece::smashboy(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);
    }

    #[test]
    fn test_rotate_clockwise_hero() {
        let origin = Point::new(6, 10);
        let piece = Piece::hero(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(8, 9),
            Point::new(8, 10),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(9, 10)
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(8, 9),
            Point::new(8, 10),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(9, 10)
        ]);
    }

    #[test]
    fn test_rotate_counterclockwise_hero() {
        let origin = Point::new(6, 10);
        let piece = Piece::hero(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(8, 9),
            Point::new(8, 10),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(9, 10)
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(8, 9),
            Point::new(8, 10),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(9, 10)
        ]);
    }

    #[test]
    fn test_rotate_clockwise_rhode_island_z() {
        let origin = Point::new(6, 10);
        let piece = Piece::rhode_island_z(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
        ]);
    }

    #[test]
    fn test_rotate_counterclockwise_rhode_island_z() {
        let origin = Point::new(6, 10);
        let piece = Piece::rhode_island_z(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
        ]);
    }

    #[test]
    fn test_rotate_clockwise_cleveland_z() {
        let origin = Point::new(6, 10);
        let piece = Piece::cleveland_z(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);
    }

    #[test]
    fn test_rotate_counterclockwise_cleveland_z() {
        let origin = Point::new(6, 10);
        let piece = Piece::cleveland_z(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12)
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(6, 11),
            Point::new(7, 11)
        ]);
    }

    #[test]
    fn test_rotate_clockwise_teewee() {
        let origin = Point::new(6, 10);
        let piece = Piece::teewee(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(7, 12)
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11)
        ]);
    }

    #[test]
    fn test_rotate_counterclockwise_teewee() {
        let origin = Point::new(6, 10);
        let piece = Piece::teewee(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(7, 12)
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(7, 12)
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(7, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11)
        ]);
    }

    #[test]
    fn test_rotate_clockwise_blue_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::blue_ricky(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(7, 12)
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(6, 12)
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(7, 12),
            Point::new(8, 12)
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(8, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11)
        ]);
    }

    #[test]
    fn test_rotate_counterclockwise_blue_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::blue_ricky(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(7, 12),
            Point::new(8, 12)
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(6, 12)
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(6, 10),
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(7, 12)
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(8, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11)
        ]);
    }

    #[test]
    fn test_rotate_clockwise_orange_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::orange_ricky(origin);
        let r90 = rotate_clockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Two);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(6, 12),
            Point::new(7, 12),
        ]);

        let r180 = rotate_clockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12),
        ]);

        let r270 = rotate_clockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Four);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(7, 12),
        ]);

        let r360 = rotate_clockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
        ])
    }

        #[test]
    fn test_rotate_counterclockwise_orange_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::orange_ricky(origin);
        let r90 = rotate_counterclockwise(&piece).unwrap();
        assert_eq!(r90.orientation, Orientation::Four);
        assert_eq!(r90.points, [
            Point::new(7, 10),
            Point::new(8, 10),
            Point::new(7, 11),
            Point::new(7, 12),
        ]);

        let r180 = rotate_counterclockwise(&r90).unwrap();
        assert_eq!(r180.orientation, Orientation::Three);
        assert_eq!(r180.points, [
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
            Point::new(8, 12),
        ]);

        let r270 = rotate_counterclockwise(&r180).unwrap();
        assert_eq!(r270.orientation, Orientation::Two);
        assert_eq!(r270.points, [
            Point::new(7, 10),
            Point::new(7, 11),
            Point::new(6, 12),
            Point::new(7, 12),
        ]);

        let r360 = rotate_counterclockwise(&r270).unwrap();
        assert_eq!(r360.orientation, Orientation::One);
        assert_eq!(r360.points, [
            Point::new(6, 10),
            Point::new(6, 11),
            Point::new(7, 11),
            Point::new(8, 11),
        ]);
    }


    #[test]
    fn test_generate_bounding_matrix_orange_ricky() {
        let origin = Point::new(6, 10);
        let piece = Piece::orange_ricky(origin);
        let matrix = generate_bounding_matrix(&piece);
        assert_eq!(matrix.width(), 3);
        assert_eq!(matrix.height(), 2);
        assert_eq!(matrix.get(0, 0), Some(1));
        assert_eq!(matrix.get(1, 0), Some(0));
        assert_eq!(matrix.get(2, 0), Some(0));
        assert_eq!(matrix.get(0, 1), Some(2));
        assert_eq!(matrix.get(1, 1), Some(3));
        assert_eq!(matrix.get(2, 1), Some(4));
    }

    #[test]
    fn test_transpose() {
        let mut matrix = Matrix::new(3, 3, 0);

        let mut val = 0;
        for y in 0..=2 {
            for x in 0..=2 {
                val += 1;
                matrix.set(x, y, val);
            }
        }

        assert_eq!(matrix.get(0, 0), Some(1));
        assert_eq!(matrix.get(1, 0), Some(2));
        assert_eq!(matrix.get(2, 0), Some(3));
        assert_eq!(matrix.get(0, 1), Some(4));
        assert_eq!(matrix.get(1, 1), Some(5));
        assert_eq!(matrix.get(2, 1), Some(6));
        assert_eq!(matrix.get(0, 2), Some(7));
        assert_eq!(matrix.get(1, 2), Some(8));
        assert_eq!(matrix.get(2, 2), Some(9));

        let transposed = transpose(matrix);

        assert_eq!(transposed.get(0, 0), Some(9));
        assert_eq!(transposed.get(1, 0), Some(6));
        assert_eq!(transposed.get(2, 0), Some(3));
        assert_eq!(transposed.get(0, 1), Some(8));
        assert_eq!(transposed.get(1, 1), Some(5));
        assert_eq!(transposed.get(2, 1), Some(2));
        assert_eq!(transposed.get(0, 2), Some(7));
        assert_eq!(transposed.get(1, 2), Some(4));
        assert_eq!(transposed.get(2, 2), Some(1));
    }
}
