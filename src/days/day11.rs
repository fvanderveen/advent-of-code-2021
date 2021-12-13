use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY11: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut grid: Grid = input.parse().unwrap();

    let result = get_flashes_after(&mut grid, 100);

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let mut grid: Grid = input.parse().unwrap();

    let result = find_step_all_flash(&mut grid);

    println!("Puzzle 2 answer: {}", result);
}

impl Grid {
    fn get_ready_to_flash(&self, already_flashed: &Vec<Point>) -> Vec<Point> {
        let mut result = vec![];

        for point in self.points() {
            match self.get(&point) {
                Some(v) if v > 9 && !already_flashed.contains(&point) => result.push(point),
                _ => {}
            }
        }

        result
    }
}

/// Runs a single flash cycle on the given `Grid`.
///
/// This method will mutate the cells inside the grid by the following steps:
/// 1. All values will be incremented by 1
/// 2. Any value that is >= 9 will cause a flash, this flash will increase adjacent values by 1 as well
/// 3. Step 2 continues until there is no flash anymore
/// 4. Any cell that flashes is set to 0
///
/// Additional rules:
/// - Any cell can only flash once per cycle
///
/// This method returns the amount of flashes happened in this cycle.
fn run_flash_cycle(grid: &mut Grid) -> usize {
    for point in grid.points() {
        match grid.get_mut(&point) {
            Some(v) => *v += 1,
            _ => {}
        }
    }

    let mut flashes = vec![];

    loop {
        let ready_to_flash = grid.get_ready_to_flash(&flashes);
        if ready_to_flash.len() == 0 {
            break;
        }

        for point in ready_to_flash {
            flashes.push(point);
            for adjacent in grid.get_adjacent_points(&point, Directions::All) {
                match grid.get_mut(&adjacent) {
                    Some(v) => *v += 1,
                    _ => {}
                }
            }
        }
    }

    for point in grid.points() {
        match grid.get_mut(&point) {
            Some(v) if *v > 9 => *v = 0,
            _ => {}
        }
    }

    flashes.len()
}

fn get_flashes_after(grid: &mut Grid, num_cycles: usize) -> usize {
    let mut result = 0;
    for _ in 0..num_cycles {
        result += run_flash_cycle(grid);
    }

    result
}

fn find_step_all_flash(grid: &mut Grid) -> usize {
    let expected_amount = grid.width * grid.height;
    let mut step = 1;
    loop {
        if run_flash_cycle(grid) == expected_amount {
            return step;
        }
        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day11::{find_step_all_flash, get_flashes_after, run_flash_cycle};
    use crate::util::geometry::Grid;

    fn get_example_grid() -> Grid {
        Grid {
            width: 10,
            height: 10,
            cells: vec![
                vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
            ],
        }
    }

    #[test]
    fn test_run_flash_cycle_small() {
        let mut grid = Grid {
            width: 5,
            height: 5,
            cells: vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 9, 1, 9, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 1, 1, 1, 1],
            ],
        };

        assert_eq!(run_flash_cycle(&mut grid), 9);
        assert_eq!(grid, Grid {
            width: 5,
            height: 5,
            cells: vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3],
            ],
        });

        assert_eq!(run_flash_cycle(&mut grid), 0);
        assert_eq!(grid, Grid {
            width: 5,
            height: 5,
            cells: vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4],
            ],
        });
    }

    #[test]
    fn test_run_flash_cycle_example() {
        let mut grid = get_example_grid();
        assert_eq!(run_flash_cycle(&mut grid), 0);
        assert_eq!(grid, Grid {
            width: 10,
            height: 10,
            cells: vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ],
        });

        assert_eq!(get_flashes_after(&mut grid, 9), 204);
        assert_eq!(grid, Grid {
            width: 10,
            height: 10,
            cells: vec![
                vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
                vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
                vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
                vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
                vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
                vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
                vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
                vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
                vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
                vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
            ],
        });

        assert_eq!(get_flashes_after(&mut grid, 90), 1452);
        assert_eq!(grid, Grid {
            width: 10,
            height: 10,
            cells: vec![
                vec![0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
                vec![0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
                vec![0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
                vec![0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
                vec![0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
                vec![0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
                vec![0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
                vec![9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
                vec![7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
                vec![6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
            ],
        });
    }

    #[test]
    fn test_find_step_all_flash()
    {
        let mut grid = get_example_grid();
        assert_eq!(find_step_all_flash(&mut grid), 195);
        assert_eq!(grid, Grid { width: 10, height: 10, cells: vec![vec![0; 10]; 10] })
    }
}