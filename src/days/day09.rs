use std::cmp::Ordering;
use crate::days::Day;
use crate::util::geometry::{Point, Grid, Directions};

pub const DAY9: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map: Grid = input.parse().unwrap();
    let values: Option<Vec<usize>> = map.find_low_spots().iter().map(|p| map.get(p).map(|v| v + 1)).collect();
    let result = values.map(|v| v.iter().sum::<usize>()).unwrap();

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    // Get the product of the three largest basins, size = number of cells
    let map: Grid = input.parse().unwrap();

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

impl Grid {
    fn find_low_spots(&self) -> Vec<Point> {
        let mut result = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let point: Point = (x, y).into();
                match self.get(&point) {
                    Some(v) => {
                        if self.get_adjacent(&point, Directions::NonDiagonal).into_iter().all(|a| v < a) {
                            result.push(point)
                        }
                    }
                    None => {}
                }
            }
        }

        result
    }

    fn get_basin(&self, p: Point) -> Vec<usize> {
        // A basin is all connected points to 'p' that are less than 9
        fn loop_get_basin(map: &Grid, current: Point, visited: &mut Vec<Point>) -> Vec<usize> {
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

            map.get_adjacent_points(&current, Directions::NonDiagonal)
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

#[cfg(test)]
mod tests {
    use crate::days::day09::{Grid};

    fn get_example_map() -> Grid {
        Grid {
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