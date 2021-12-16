use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY15: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let grid: Grid = input.parse().unwrap();

    let score = find_lowest_risk_path_cost(&grid).unwrap();

    println!("Puzzle 1 answer: {}", score);
}

fn puzzle2(input: &String) {
    let grid: Grid = input.parse().unwrap();
    let no_the_real_grid = build_real_map(&grid);

    let score = find_lowest_risk_path_cost(&no_the_real_grid).unwrap();

    println!("Puzzle 2 answer: {}", score);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct DijkstraEntry {
    distance: usize,
    node: Point,
}

impl Ord for DijkstraEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| self.node.x.cmp(&other.node.x))
            .then_with(|| self.node.y.cmp(&other.node.y))
    }
}

impl PartialOrd for DijkstraEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_lowest_risk_path_cost(grid: &Grid) -> Option<usize> {
    // Start at top-left, end at bottom-right
    // How bad will brute force perform? -> Very, very bad. 😂
    // Since this is pretty much shortest path, let's see if I can build Dijkstra.

    fn dijkstra(grid: &Grid, target: &Point) -> Option<usize> {
        let mut dist = HashMap::new();
        let mut queue = BinaryHeap::new();

        let start: Point = (0, 0).into();
        dist.insert(start, 0 as usize);
        queue.push(DijkstraEntry { node: start, distance: 0 });

        while let Some(DijkstraEntry { node, distance }) = queue.pop() {
            if node.eq(target) {
                // We found the part to target, we're done.
                return Some(distance);
            }

            // If there already is a distance for this node that is less, we can skip this
            if let Some(current_distance) = dist.get(&node) {
                if distance.gt(current_distance) {
                    continue;
                }
            }

            for neighbor in grid.get_adjacent_points(&node, Directions::NonDiagonal) {
                let distance_to_neighbor = grid.get(&neighbor).unwrap();
                let total_distance_through_current = distance + distance_to_neighbor;
                let current_distance = dist.get(&neighbor).unwrap_or(&usize::MAX);

                if total_distance_through_current.lt(current_distance) {
                    queue.push(DijkstraEntry { node: neighbor.clone(), distance: total_distance_through_current });
                    dist.insert(neighbor.clone(), total_distance_through_current);
                }
            }
        }

        None
    }

    let target = Point { x: grid.width - 1, y: grid.height - 1 };
    dijkstra(grid, &target)
}

fn build_real_map(segment: &Grid) -> Grid {
    // The real map is apparently 5 times as large. Here's hoping I did dijkstra right for puzzle 1.

    // To stitch the real map together, we copy the initial segment to the right/bottom, while increasing
    // all costs by 1. A 9 will go back to 1.

    let height = segment.height * 5;
    let width = segment.width * 5;
    let mut cells: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for x in 0..5 {
        for y in 0..5 {
            for point in segment.points() {
                let new_x = point.x + x * segment.width;
                let new_y = point.y + y * segment.height;
                let value = segment.get(&point).unwrap_or(0) + x + y;
                cells[new_y][new_x] = if value > 9 { value % 9 } else { value }
            }
        }
    }

    Grid { height, width, cells }
}

#[cfg(test)]
mod tests {
    use crate::days::day15::{build_real_map, find_lowest_risk_path_cost};
    use crate::util::geometry::{Grid};

    const EXAMPLE_INPUT: &str = "\
        1163751742\n\
        1381373672\n\
        2136511328\n\
        3694931569\n\
        7463417111\n\
        1319128137\n\
        1359912421\n\
        3125421639\n\
        1293138521\n\
        2311944581";

    #[test]
    fn test_find_lowest_risk_path_cost() {
        let grid: Grid = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(find_lowest_risk_path_cost(&grid), Some(40));
    }

    #[test]
    fn test_build_real_map() {
        let grid = Grid { width: 1, height: 1, cells: vec![vec![8]] };

        let big_grid = build_real_map(&grid);
        assert_eq!(big_grid, Grid {
            width: 5,
            height: 5,
            cells: vec![
                vec![8, 9, 1, 2, 3],
                vec![9, 1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
                vec![2, 3, 4, 5, 6],
                vec![3, 4, 5, 6, 7]
            ]
        });

        let big_example_grid = build_real_map(&EXAMPLE_INPUT.parse().unwrap());
        assert_eq!(big_example_grid, Grid {
            width: 50,
            height: 50,
            cells: vec![
                vec![1,1,6,3,7,5,1,7,4,2,2,2,7,4,8,6,2,8,5,3,3,3,8,5,9,7,3,9,6,4,4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6],
                vec![1,3,8,1,3,7,3,6,7,2,2,4,9,2,4,8,4,7,8,3,3,5,1,3,5,9,5,8,9,4,4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6],
                vec![2,1,3,6,5,1,1,3,2,8,3,2,4,7,6,2,2,4,3,9,4,3,5,8,7,3,3,5,4,1,5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3],
                vec![3,6,9,4,9,3,1,5,6,9,4,7,1,5,1,4,2,6,7,1,5,8,2,6,2,5,3,7,8,2,6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4],
                vec![7,4,6,3,4,1,7,1,1,1,8,5,7,4,5,2,8,2,2,2,9,6,8,5,6,3,9,3,3,3,1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5],
                vec![1,3,1,9,1,2,8,1,3,7,2,4,2,1,2,3,9,2,4,8,3,5,3,2,3,4,1,3,5,9,4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2],
                vec![1,3,5,9,9,1,2,4,2,1,2,4,6,1,1,2,3,5,3,2,3,5,7,2,2,3,4,6,4,3,4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5],
                vec![3,1,2,5,4,2,1,6,3,9,4,2,3,6,5,3,2,7,4,1,5,3,4,7,6,4,3,8,5,2,6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4],
                vec![1,2,9,3,1,3,8,5,2,1,2,3,1,4,2,4,9,6,3,2,3,4,2,5,3,5,1,7,4,3,4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5],
                vec![2,3,1,1,9,4,4,5,8,1,3,4,2,2,1,5,5,6,9,2,4,5,3,3,2,6,6,7,1,3,5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5],
                vec![2,2,7,4,8,6,2,8,5,3,3,3,8,5,9,7,3,9,6,4,4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7],
                vec![2,4,9,2,4,8,4,7,8,3,3,5,1,3,5,9,5,8,9,4,4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7],
                vec![3,2,4,7,6,2,2,4,3,9,4,3,5,8,7,3,3,5,4,1,5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4],
                vec![4,7,1,5,1,4,2,6,7,1,5,8,2,6,2,5,3,7,8,2,6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5],
                vec![8,5,7,4,5,2,8,2,2,2,9,6,8,5,6,3,9,3,3,3,1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6],
                vec![2,4,2,1,2,3,9,2,4,8,3,5,3,2,3,4,1,3,5,9,4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3],
                vec![2,4,6,1,1,2,3,5,3,2,3,5,7,2,2,3,4,6,4,3,4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6],
                vec![4,2,3,6,5,3,2,7,4,1,5,3,4,7,6,4,3,8,5,2,6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5],
                vec![2,3,1,4,2,4,9,6,3,2,3,4,2,5,3,5,1,7,4,3,4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6],
                vec![3,4,2,2,1,5,5,6,9,2,4,5,3,3,2,6,6,7,1,3,5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6],
                vec![3,3,8,5,9,7,3,9,6,4,4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7,7,7,3,9,4,2,7,4,1,8],
                vec![3,5,1,3,5,9,5,8,9,4,4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7,7,9,5,7,9,4,9,3,4,8],
                vec![4,3,5,8,7,3,3,5,4,1,5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4,8,7,9,3,2,7,7,9,8,5],
                vec![5,8,2,6,2,5,3,7,8,2,6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5,9,3,6,1,6,9,7,2,3,6],
                vec![9,6,8,5,6,3,9,3,3,3,1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6,4,1,3,9,1,7,4,7,7,7],
                vec![3,5,3,2,3,4,1,3,5,9,4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3,7,9,7,6,7,8,5,7,9,4],
                vec![3,5,7,2,2,3,4,6,4,3,4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6,7,9,2,6,6,7,8,1,8,7],
                vec![5,3,4,7,6,4,3,8,5,2,6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5,9,7,8,2,1,8,7,3,9,6],
                vec![3,4,2,5,3,5,1,7,4,3,4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6,7,8,6,9,7,9,5,2,8,7],
                vec![4,5,3,3,2,6,6,7,1,3,5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6,8,9,7,7,6,1,1,2,5,7],
                vec![4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7,7,7,3,9,4,2,7,4,1,8,8,8,4,1,5,3,8,5,2,9],
                vec![4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7,7,9,5,7,9,4,9,3,4,8,8,1,6,8,1,5,1,4,5,9],
                vec![5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4,8,7,9,3,2,7,7,9,8,5,9,8,1,4,3,8,8,1,9,6],
                vec![6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5,9,3,6,1,6,9,7,2,3,6,1,4,7,2,7,1,8,3,4,7],
                vec![1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6,4,1,3,9,1,7,4,7,7,7,5,2,4,1,2,8,5,8,8,8],
                vec![4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3,7,9,7,6,7,8,5,7,9,4,8,1,8,7,8,9,6,8,1,5],
                vec![4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6,7,9,2,6,6,7,8,1,8,7,8,1,3,7,7,8,9,2,9,8],
                vec![6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5,9,7,8,2,1,8,7,3,9,6,1,8,9,3,2,9,8,4,1,7],
                vec![4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6,7,8,6,9,7,9,5,2,8,7,8,9,7,1,8,1,6,3,9,8],
                vec![5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6,8,9,7,7,6,1,1,2,5,7,9,1,8,8,7,2,2,3,6,8],
                vec![5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7,7,7,3,9,4,2,7,4,1,8,8,8,4,1,5,3,8,5,2,9,9,9,5,2,6,4,9,6,3,1],
                vec![5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7,7,9,5,7,9,4,9,3,4,8,8,1,6,8,1,5,1,4,5,9,9,2,7,9,2,6,2,5,6,1],
                vec![6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4,8,7,9,3,2,7,7,9,8,5,9,8,1,4,3,8,8,1,9,6,1,9,2,5,4,9,9,2,1,7],
                vec![7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5,9,3,6,1,6,9,7,2,3,6,1,4,7,2,7,1,8,3,4,7,2,5,8,3,8,2,9,4,5,8],
                vec![2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6,4,1,3,9,1,7,4,7,7,7,5,2,4,1,2,8,5,8,8,8,6,3,5,2,3,9,6,9,9,9],
                vec![5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3,7,9,7,6,7,8,5,7,9,4,8,1,8,7,8,9,6,8,1,5,9,2,9,8,9,1,7,9,2,6],
                vec![5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6,7,9,2,6,6,7,8,1,8,7,8,1,3,7,7,8,9,2,9,8,9,2,4,8,8,9,1,3,1,9],
                vec![7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5,9,7,8,2,1,8,7,3,9,6,1,8,9,3,2,9,8,4,1,7,2,9,1,4,3,1,9,5,2,8],
                vec![5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6,7,8,6,9,7,9,5,2,8,7,8,9,7,1,8,1,6,3,9,8,9,1,8,2,9,2,7,4,1,9],
                vec![6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6,8,9,7,7,6,1,1,2,5,7,9,1,8,8,7,2,2,3,6,8,1,2,9,9,8,3,3,4,7,9]
            ]
        });
    }

    #[test]
    fn test_find_lowest_risk_on_real_map() {
        let grid: Grid = EXAMPLE_INPUT.parse().unwrap();
        let real_grid = build_real_map(&grid);

        assert_eq!(find_lowest_risk_path_cost(&real_grid), Some(315));
    }
}