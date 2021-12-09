use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::Point;
use crate::util::number;

pub const DAY9: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map: HeightMap = input.parse().unwrap();
    let values: Option<Vec<u32>> = map.find_low_spots().iter().map(|p| map.get(p).map(|v| v + 1)).collect();
    let result = values.map(|v| v.iter().sum::<u32>()).unwrap();

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    // Get the product of the three largest basins, size = number of cells
    let map: HeightMap = input.parse().unwrap();

    let mut basins: Vec<usize> = map.find_low_spots().into_iter().map(|p| map.get_basin(p).len()).collect();
    // Sort inverted
    basins.sort_by(|a, b| match a.cmp(b) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => Ordering::Equal
    });

    let result = basins[0..3].iter().fold(1, |a, b| a * b);
    println!("Puzzle 2 answer: {}", result);
}


#[derive(Eq, PartialEq)]
struct HeightMap {
    width: usize,
    height: usize,
    cells: Vec<Vec<u32>>,
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeightMap")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("map", &format_args!("{}", &self))
            .finish()
    }
}

impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lines: Vec<String> = self.cells.iter().map(|l| l.iter().map(|v| format!("{}", v)).collect::<String>()).collect();
        write!(f, "{}", lines.join("\n"))
    }
}

impl HeightMap {
    fn find_low_spots(&self) -> Vec<Point> {
        let mut result = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let point: Point = (x, y).into();
                match self.get(&point) {
                    Some(v) => {
                        if self.get_adjacent(&point).into_iter().all(|a| v < a) {
                            result.push(point)
                        }
                    }
                    None => {}
                }
            }
        }

        result
    }

    fn get(&self, p: &Point) -> Option<u32> {
        self.cells.get(p.y).and_then(|r| r.get(p.x)).map(|x| x.clone())
    }

    fn get_adjacent(&self, p: &Point) -> Vec<u32> {
        self.get_adjacent_points(p).iter().map(|p| self.get(p).unwrap()).collect()
    }

    fn get_adjacent_points(&self, p: &Point) -> Vec<Point> {
        let mut points = vec![];

        let x = p.x;
        let y = p.y;

        if y > 0 { points.push((x, y - 1).into()) } // top
        if x < self.width - 1 { points.push((x + 1, y).into()) } // right
        if y < self.height - 1 { points.push((x, y + 1).into()) } // bottom
        if x > 0 { points.push((x - 1, y).into()) } // left

        points
    }

    fn get_basin(&self, p: Point) -> Vec<u32> {
        // A basin is all connected points to 'p' that are less than 9
        fn loop_get_basin(map: &HeightMap, current: Point, visited: &mut Vec<Point>) -> Vec<u32> {
            let mut result = vec![];
            if visited.contains(&current) {
                return result;
            }

            match map.get(&current) {
                None => return result,
                Some(v) if v >= 9 => return result,
                Some(v) => { result.push(v) }
            }

            visited.push(current);

            map.get_adjacent_points(&current)
                .into_iter()
                .filter(|v| match map.get(v) {
                    None => false,
                    Some(v) => v < 9
                })
                .flat_map(|v| loop_get_basin(map, v, visited))
                .for_each(|v| result.push(v));

            result
        }

        loop_get_basin(self, p, &mut vec![])
    }
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Vec<Vec<u32>>, String> = s.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| number::parse_u32(c.to_string().as_str())).collect::<Result<Vec<u32>, String>>())
            .collect();

        let cells = match parse_result {
            Ok(lines) if lines.len() == 0 => {
                return Ok(HeightMap { width: 0, height: 0, cells: vec![] });
            }
            Ok(lines) => lines,
            Err(e) => return Err(e)
        };

        let height = cells.len();
        let width = cells[0].len();
        if !cells.iter().all(|l| l.len() == width) {
            return Err(format!("Not all lines in input are the same width"));
        }

        Ok(HeightMap { width, height, cells })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{HeightMap};

    const EXAMPLE_INPUT: &str = "\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\
    ";

    fn get_example_map() -> HeightMap {
        HeightMap {
            width: 10,
            height: 5,
            cells: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        }
    }

    #[test]
    fn test_map_debug() {
        assert_eq!(format!("{:?}", get_example_map()),
                   "HeightMap { \
                       width: 10, \
                       height: 5, \
                       map: 2199943210\n\
                            3987894921\n\
                            9856789892\n\
                            8767896789\n\
                            9899965678 \
                   }");
    }

    #[test]
    fn test_map_format() {
        assert_eq!(format!("{}", get_example_map()), EXAMPLE_INPUT);
    }

    #[test]
    fn test_map_from_str() {
        assert_eq!(EXAMPLE_INPUT.parse::<HeightMap>(), Ok(get_example_map()));
    }

    #[test]
    fn test_get_adjacent() {
        let map = get_example_map();
        assert_eq!(map.get_adjacent(&(0, 0).into()), vec![1, 3]);
        assert_eq!(map.get_adjacent(&(5, 0).into()), vec![3, 9, 9]);
        assert_eq!(map.get_adjacent(&(5, 3).into()), vec![8, 6, 6, 8]);
        assert_eq!(map.get_adjacent(&(9, 4).into()), vec![9, 7]);
    }

    #[test]
    fn test_find_low_spots() {
        let map = get_example_map();
        let low_spots = map.find_low_spots();
        assert_eq!(low_spots, vec![
            (1, 0).into(),
            (9, 0).into(),
            (2, 2).into(),
            (6, 4).into(),
        ]);
    }

    #[test]
    fn test_get_basin() {
        let map = get_example_map();

        assert_eq!(map.get_basin((1, 0).into()), vec![1, 2, 3]);
        assert_eq!(map.get_basin((9, 0).into()), vec![0, 1, 2, 2, 1, 2, 3, 4, 4]);
        assert_eq!(map.get_basin((2, 2).into()), vec![5, 8, 7, 8, 7, 8, 8, 7, 6, 6, 7, 8, 8, 8]);
        assert_eq!(map.get_basin((6, 4).into()), vec![5, 6, 7, 8, 8, 7, 8, 6, 6]);
    }
}