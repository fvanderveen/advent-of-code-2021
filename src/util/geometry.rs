use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::ops::{Range};
use std::str::FromStr;
use crate::util::number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from(p: (isize, isize)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl TryFrom<(usize, usize)> for Point {
    type Error = String;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let x: isize = isize::try_from(value.0).map_err(|e| format!("{}", e))?;
        let y: isize = isize::try_from(value.1).map_err(|e| format!("{}", e))?;
        Ok(Point { x, y })
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts_result: Result<Vec<isize>, String> = s.split(",").map(|p| number::parse_isize(p)).collect();
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

#[cfg(test)]
mod point_tests {
    use crate::util::geometry::Point;

    #[test]
    fn test_from_str() {
        assert_eq!("3,5".parse(), Ok(Point { x: 3, y: 5 }));
        assert_eq!("3,-5".parse(), Ok(Point { x: 3, y: -5 }));
        assert_eq!("422,-2345".parse(), Ok(Point { x: 422, y: -2345 }));
    }

    #[test]
    fn test_from() {
        assert_eq!(Point::from((3, 5)), Point { x: 3, y: 5 });
        assert_eq!(Point::from((-42, -10)), Point { x: -42, y: -10 });
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", Point { x: 5, y: -10 }), "(5,-10)");
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    fn length(&self) -> usize {
        let x1 = self.start.x;
        let x2 = self.end.x;

        (x1 - x2).abs() as usize
    }

    fn height(&self) -> usize {
        let y1 = self.start.y;
        let y2 = self.end.y;

        (y1 - y2).abs() as usize
    }

    fn dx(&self) -> isize {
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

    fn dy(&self) -> isize {
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

    fn x(&self, t: usize) -> isize {
        let step: isize = (t as isize) * self.dx();
        self.start.x + step
    }

    fn y(&self, t: usize) -> isize {
        let step: isize = (t as isize) * self.dy();
        self.start.y + step
    }

    pub fn get_points(&self) -> Vec<Point> {
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
mod line_tests {
    use crate::util::geometry::{Line, Point};

    const fn point(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    const fn line(x1: isize, y1: isize, x2: isize, y2: isize) -> Line {
        Line { start: point(x1, y1), end: point(x2, y2) }
    }

    #[test]
    fn test_get_points() {
        assert_eq!(line(12, 0, 12, 6).get_points(), vec![point(12, 0), point(12, 1), point(12, 2), point(12, 3), point(12, 4), point(12, 5), point(12, 6)]);
        assert_eq!(line(2, 2, 4, 4).get_points(), vec![point(2, 2), point(3, 3), point(4, 4)]);
        assert_eq!(line(4, 0, 2, 0).get_points(), vec![point(4, 0), point(3, 0), point(2, 0)]);
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Bounds {
    pub top: isize,
    pub left: isize,
    pub width: usize,
    pub height: usize,
}

#[allow(unused)]
impl Bounds {
    pub fn from_tlbr(top: isize, left: isize, bottom: isize, right: isize) -> Self {
        Self {
            top,
            left,
            width: (right - left).max(0) as usize + 1,
            height: (bottom - top).max(0) as usize + 1,
        }
    }

    pub fn grow(&mut self, by: isize) {
        self.top -= by;
        self.left -= by;
        self.width = (self.width as isize + 2 * by) as usize;
        self.height = (self.height as isize + 2 * by) as usize
    }

    pub fn y(&self) -> Range<isize> {
        self.top..self.bottom()
    }

    pub fn x(&self) -> Range<isize> {
        self.left..self.right()
    }

    pub fn right(&self) -> isize {
        self.left + self.width as isize
    }

    pub fn bottom(&self) -> isize {
        self.top + self.height as isize
    }

    pub fn contains(&self, pixel: &Point) -> bool {
        self.x().contains(&pixel.x) && self.y().contains(&pixel.y)
    }
}

#[derive(Eq, PartialEq, Clone, Default)]
pub struct Grid<T> where T: Clone + Default {
    pub bounds: Bounds,
    cells: HashMap<Point, T>,
}

#[repr(u8)]
#[derive(Eq, PartialEq, Clone)]
pub enum Directions {
    Horizontal = 1,
    Vertical = 2,
    Diagonal = 4,
    NonDiagonal = 3,
    All = 7
}

impl Directions {
    fn has(&self, value: Directions) -> bool {
        (self.clone() as u8 & value as u8) != 0
    }
}

#[allow(unused)]
impl<T> Grid<T> where T: Clone + Default {
    pub fn new(cells: HashMap<Point, T>) -> Self {
        let points: Vec<_> = cells.keys().collect();
        let top = points.iter().map(|p| p.y).min().unwrap_or(0);
        let bottom = points.iter().map(|p| p.y).max().unwrap_or(0);
        let left = points.iter().map(|p| p.x).min().unwrap_or(0);
        let right = points.iter().map(|p| p.x).max().unwrap_or(0);

        let bounds = Bounds::from_tlbr(top, left, bottom, right);
        Self { bounds, cells }
    }

    pub fn get(&self, p: &Point) -> Option<T> {
        self.cells.get(p).map(|x| x.clone())
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> {
        self.cells.get_mut(p)
    }

    pub fn get_adjacent(&self, p: &Point, directions: Directions) -> Vec<T> {
        self.get_adjacent_points(p, directions).iter().filter_map(|p| self.get(p)).collect()
    }

    pub fn get_adjacent_points(&self, p: &Point, directions: Directions) -> Vec<Point> {
        let mut points = vec![];

        let x = p.x;
        let y = p.y;

        let include_vertical = directions.has(Directions::Vertical);
        let include_horizontal = directions.has(Directions::Horizontal);
        let include_diagonal = directions.has(Directions::Diagonal);

        let include_top = y > self.bounds.top;
        let include_bottom = y < self.bounds.bottom() - 1;
        let include_left = x > self.bounds.left;
        let include_right = x < self.bounds.right() - 1;

        if include_top && include_horizontal { points.push((x, y - 1).into()) } // top
        if include_top && include_right && include_diagonal { points.push((x + 1, y - 1).into()) } // top-right
        if include_right && include_horizontal { points.push((x + 1, y).into()) } // right
        if include_bottom && include_right && include_diagonal { points.push((x + 1, y + 1).into()) } // bottom-right
        if include_bottom && include_vertical { points.push((x, y + 1).into()) } // bottom
        if include_bottom && include_left && include_diagonal { points.push((x - 1, y + 1).into()) } // bottom-left
        if include_left && include_horizontal { points.push((x - 1, y).into()) } // left
        if include_left && include_top && include_diagonal { points.push((x - 1, y - 1).into()) } // top-left

        points
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        for y in self.bounds.y() {
            for x in self.bounds.x() {
                points.push((x, y).into());
            }
        }

        points
    }

    pub fn values(&self) -> Vec<T> {
        self.points().iter().map(|p| self.get(p).unwrap_or_default()).collect()
    }
}

impl<T> fmt::Debug for Grid<T> where T: fmt::Display + Clone + Default {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("bounds", &self.bounds)
            .field("map", &format_args!("{:,>}", &self))
            .finish()
    }
}

impl<T> fmt::Display for Grid<T> where T: fmt::Display + Clone + Default {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![];

        for y in self.bounds.y() {
            let mut line = vec![];
            for x in self.bounds.x() {
                line.push(self.cells.get(&(x, y).into()).map(|v| format!("{}", v)).unwrap_or(String::new()))
            }
            lines.push(line);
        }

        let cell_width = lines.iter().map(|line| line.iter().map(|v| v.len()).max().unwrap_or(0)).max().unwrap_or(0);

        let fill = f.fill().to_string();
        let formatted_lines: Vec<_> = lines.iter().map(|line| {
            let formatted_line: Vec<_> = line.iter().map(|v| " ".repeat(cell_width - v.len()) + v).collect();
            formatted_line.join(if f.align().is_some() { fill.as_str() } else { "" })
        }).collect();

        write!(f, "{}", formatted_lines.join("\n"))
    }
}

impl<T> FromStr for Grid<T> where T: FromStr + Clone + Default {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Vec<Vec<T>>, String> = s.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c|
                String::from(c).parse::<T>().map_err(|_| format!("Could not parse '{}' to {}", c, std::any::type_name::<T>())))
                .collect::<Result<Vec<T>, String>>())
            .collect();

        let cells = match parse_result {
            Ok(lines) if lines.len() == 0 => {
                return Ok(Grid::default());
            }
            Ok(lines) => lines,
            Err(e) => return Err(e)
        };

        Grid::try_from(cells)
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> where T: Clone + Default {
    type Error = String;

    fn try_from(data: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = data.len();
        let width = data[0].len();

        let bounds = Bounds { top: 0, left: 0, width, height };

        if data.iter().all(|l| l.len() == width) {
            let mut cells = HashMap::new();
            for y in 0..height {
                for x in 0..width {
                    cells.insert((x, y).try_into().unwrap(), data[y][x].clone());
                }
            }

            Ok(Grid { bounds, cells })
        } else {
            Err(format!("Not all lines in input are the same width"))
        }
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::util::geometry::{Grid, Directions};

    const EXAMPLE_GRID_INPUT: &str = "\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\
    ";

    fn get_example_grid() -> Grid<usize> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ].try_into().unwrap()
    }

    #[test]
    fn test_grid_debug() {
        assert_eq!(format!("{:?}", get_example_grid()),
                   "Grid { \
                       bounds: Bounds { top: 0, left: 0, width: 10, height: 5 }, \
                       map: 2,1,9,9,9,4,3,2,1,0\n\
                            3,9,8,7,8,9,4,9,2,1\n\
                            9,8,5,6,7,8,9,8,9,2\n\
                            8,7,6,7,8,9,6,7,8,9\n\
                            9,8,9,9,9,6,5,6,7,8 \
                   }");
    }

    #[test]
    fn test_grid_format() {
        assert_eq!(format!("{}", get_example_grid()), EXAMPLE_GRID_INPUT);
        assert_eq!(format!("{:|^}", get_example_grid()), "\
            2|1|9|9|9|4|3|2|1|0\n\
            3|9|8|7|8|9|4|9|2|1\n\
            9|8|5|6|7|8|9|8|9|2\n\
            8|7|6|7|8|9|6|7|8|9\n\
            9|8|9|9|9|6|5|6|7|8");
    }

    #[test]
    fn test_grid_from_str() {
        assert_eq!(EXAMPLE_GRID_INPUT.parse::<Grid<usize>>(), Ok(get_example_grid()));
    }

    #[test]
    fn test_get_adjacent() {
        let grid = get_example_grid();
        assert_eq!(grid.get_adjacent(&(0, 0).into(), Directions::NonDiagonal), vec![1, 3]);
        assert_eq!(grid.get_adjacent(&(0, 0).into(), Directions::All), vec![1, 9, 3]);
        assert_eq!(grid.get_adjacent(&(5, 0).into(), Directions::NonDiagonal), vec![3, 9, 9]);
        assert_eq!(grid.get_adjacent(&(5, 3).into(), Directions::NonDiagonal), vec![8, 6, 6, 8]);
        assert_eq!(grid.get_adjacent(&(9, 4).into(), Directions::NonDiagonal), vec![9, 7]);
    }

    #[test]
    fn test_get_adjacent_points() {
        let grid = get_example_grid();

        assert_eq!(grid.get_adjacent_points(&(0, 0).into(), Directions::NonDiagonal), vec![(1, 0).into(), (0, 1).into()]);
        assert_eq!(grid.get_adjacent_points(&(0, 0).into(), Directions::All), vec![(1, 0).into(), (1, 1).into(), (0, 1).into()]);

        assert_eq!(grid.get_adjacent_points(&(5, 3).into(), Directions::NonDiagonal),
                   vec![(5, 2).into(), (6, 3).into(), (5, 4).into(), (4, 3).into()]);
        assert_eq!(grid.get_adjacent_points(&(5, 3).into(), Directions::All),
                   vec![(5, 2).into(), (6, 2).into(), (6, 3).into(), (6, 4).into(), (5, 4).into(), (4, 4).into(), (4, 3).into(), (4, 2).into()]);
    }

    #[test]
    fn test_values() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![9, 8, 7], vec![5, 6, 4]].try_into().unwrap();
        assert_eq!(grid.values(), vec![1, 2, 3, 9, 8, 7, 5, 6, 4]);
    }
}