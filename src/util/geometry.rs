use std::cmp::max;
use std::fmt;
use std::str::FromStr;
use crate::util::number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts_result: Result<Vec<usize>, String> = s.split(",").map(|p| number::parse_usize(p)).collect();
        let parts = match parts_result {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        match parts.len() {
            2 => Ok((parts[0], parts[1]).into()),
            _ => Err(format!("Invalid str format for Point '{}', expected 'x,y'", s))
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub trait GetPoints {
    fn get_points(&self) -> Vec<Point>;
}

impl Line {
    fn length(&self) -> usize {
        let x1 = self.start.x;
        let x2 = self.end.x;

        if x1 > x2 { x1 - x2 } else { x2 - x1 }
    }
    fn height(&self) -> usize {
        let y1 = self.start.y;
        let y2 = self.end.y;

        if y1 > y2 { y1 - y2 } else { y2 - y1 }
    }
    fn dx(&self) -> i8 {
        let x1 = self.start.x;
        let x2 = self.end.x;

        if x1 == x2 {
            0
        } else if x2 > x1 {
            1
        } else {
            -1
        }
    }
    fn dy(&self) -> i8 {
        let y1 = self.start.y;
        let y2 = self.end.y;

        if y1 == y2 {
            0
        } else if y2 > y1 {
            1
        } else {
            -1
        }
    }
    fn x(&self, t: usize) -> usize {
        let step: isize = (t as isize) * self.dx() as isize;
        (self.start.x as isize + step) as usize
    }
    fn y(&self, t: usize) -> usize {
        let step: isize = (t as isize) * self.dy() as isize;
        (self.start.y as isize + step) as usize
    }
}

impl GetPoints for Line {
    fn get_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let length = self.length();
        let height = self.height();

        if length == 0 && height == 0 {
            return points;
        }

        // Given the puzzle, the lines seem to be either horizontal, vertical, or 45deg.
        // We'll panic for any other case for now.
        if length != 0 && height != 0 && length != height {
            panic!("Cannot get points for line {:?}", self);
        }

        let steps = max(length, height);
        for i in 0..steps + 1 {
            points.push((self.x(i), self.y(i)).into());
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use crate::util::geometry::{GetPoints, Line, Point};

    const fn point(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    const fn line(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        Line { start: point(x1, y1), end: point(x2, y2) }
    }

    #[test]
    fn test_get_points() {
        assert_eq!(line(12, 0, 12, 6).get_points(), vec![point(12, 0), point(12, 1), point(12, 2), point(12, 3), point(12, 4), point(12, 5), point(12, 6)]);
        assert_eq!(line(2, 2, 4, 4).get_points(), vec![point(2, 2), point(3, 3), point(4, 4)]);
        assert_eq!(line(4, 0, 2, 0).get_points(), vec![point(4, 0), point(3, 0), point(2, 0)]);
    }
}