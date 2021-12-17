use std::ops::{RangeInclusive};
use std::str::FromStr;
use regex::Regex;
use crate::days::Day;
use crate::util::number::parse_isize;

pub const DAY17: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let area: TargetArea = input.parse().unwrap();
    let trajectory = calculate_highest_trajectory(&area).unwrap();
    let result = trajectory.get_top();

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let area: TargetArea = input.parse().unwrap();
    let trajectories = get_all_possible_trajectories(&area).unwrap();

    println!("Puzzle 2 answer: {}", trajectories.len());
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct TargetArea {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl TargetArea {
    fn bottom(&self) -> isize {
        self.y.end().min(self.y.start()).clone()
    }

    fn left(&self) -> isize {
        self.x.start().min(self.x.end()).clone()
    }

    fn right(&self) -> isize {
        self.x.start().max(self.x.end()).clone()
    }
}

impl FromStr for TargetArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new("^target area: x=(?P<x1>-?\\d+)..(?P<x2>-?\\d+), y=(?P<y1>-?\\d+)..(?P<y2>-?\\d+)$").map_err(|e| format!("{}", e))?;

        let captures = regex.captures(s).ok_or(format!("Couldn't parse input target: {}", s))?;
        let x1 = captures.name("x1").ok_or(format!("Missing x1 in {}", s)).and_then(|v| parse_isize(v.as_str()))?;
        let x2 = captures.name("x2").ok_or(format!("Missing x2 in {}", s)).and_then(|v| parse_isize(v.as_str()))?;
        let y1 = captures.name("y1").ok_or(format!("Missing y1 in {}", s)).and_then(|v| parse_isize(v.as_str()))?;
        let y2 = captures.name("y2").ok_or(format!("Missing y2 in {}", s)).and_then(|v| parse_isize(v.as_str()))?;

        Ok(TargetArea { x: x1..=x2, y: y1..=y2 })
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Trajectory {
    x: isize,
    y: isize,
}

impl Trajectory {
    fn get_top(&self) -> isize {
        // See below for this math function.
        (self.y * (self.y + 1)) / 2
    }
}

impl From<(isize, isize)> for Trajectory {
    fn from((x, y): (isize, isize)) -> Self {
        Trajectory { x, y }
    }
}

fn get_steps_for_y_hit(area: &TargetArea, initial_y: isize) -> Option<RangeInclusive<isize>> {
    // Get t for any y:
    // - if y is positive, it takes 2*y steps to get to 0, and then n steps to get inside the area
    // - if y is negative, it takes n steps to get inside the area.
    // To get this n:
    // just loop a few times. I'm done doing calculus.

    let mut dy = -(initial_y.abs());
    let mut y = 0;

    let mut steps: isize = if initial_y > 0 { 2 * initial_y } else { 0 };

    let mut min_y_hit = None;
    let mut max_y_hit = None;

    loop {
        steps += 1;
        y += dy;

        if area.y.contains(&y) {
            min_y_hit = min_y_hit.or(Some(steps));
            max_y_hit = max_y_hit.or(Some(steps)).and_then(|v| Some(v.max(steps)));
        };
        if y < area.bottom() {
            break;
        }

        dy -= 1;
    }

    match (min_y_hit, max_y_hit) {
        (Some(min), Some(max)) => Some(min..=max),
        _ => None
    }
}

fn get_xs_for_y(area: &TargetArea, y: isize) -> Option<Vec<isize>> {
    // To find an X, we need to know how many steps it will take for the trajectory to hit on the Y axis
    // This is 2*Y+1 (as dy will go to 0 in y steps, and needs another y steps to get back at Y=0, plus one more to hit)
    // To find the initial X, we need _a_ value that will be inside the X of the area after the amount of steps.
    // Given an initial X and number of steps t, X't should be:
    // N = min(X,t)
    // S = X
    // E = max(X-(t-1), 1)
    // N(S+E)/2
    // We know t. We need to figure an X such that the above solved to a value within the area's X range.
    let mut results = vec![];

    let steps_range = get_steps_for_y_hit(area, y)?;

    let mut x_guess = area.left() / steps_range.end(); // Since we have drag, this _should_ end up far before the target.

    loop {
        let xts: Vec<isize> = steps_range.clone().map(|t| {
            let n = x_guess.min(t);
            let start = x_guess;
            let end = (x_guess - (t - 1)).max(1);
            (n * (start + end)) / 2
        }).collect();

        // If all resulting X's are beyond the range, we're done.
        if xts.iter().all(|x| area.right().lt(x)) {
            break;
        }

        // Otherwise, if any of the xts is inside the range, add it as result
        if xts.iter().any(|x| area.x.contains(x)) {
            results.push(x_guess);
        }

        x_guess += 1;
    }

    Some(results)
}

fn calculate_highest_trajectory(area: &TargetArea) -> Option<Trajectory> {
    // Find a trajectory that reaches the highest Y value, and still hits area.
    // Technically, shooting upwards will mirror on the way back, due to gravity being a constant -1 on y.
    // This means that we reach Y=0 with the same downward speed as we started upwards.
    // Practically, the puzzle only has negative Y values, which makes this a little simpler:
    // Since we cross Y=0 with the initial upwards speed; it means that in order to _hit_ the target,
    // we need a speed that will hit the very bottom of the target in the next step.
    // In other words, the upward Y will be abs(<area.y.bottom - 1>)
    let y_target = area.bottom();
    let y = (y_target + 1).abs();

    let xs = get_xs_for_y(area, y)?;
    xs.first().cloned().map(|x| Trajectory { x, y })
}

fn get_all_possible_trajectories(area: &TargetArea) -> Option<Vec<Trajectory>> {
    // We at least know the bounds of Y.
    // The max can be found with the highest trajectory.
    let max_y = calculate_highest_trajectory(area)?.y;
    // The lowest is the value that ends up at the bottom of area in one step.
    let min_y = area.bottom();

    let mut results = vec![];

    for y in min_y..=max_y {
        if let Some(xs) = get_xs_for_y(area, y) {
            for x in xs {
                results.push(Trajectory { x, y });
            }
        }
    }

    Some(results)
}

#[cfg(test)]
mod tests {
    use crate::days::day17::{calculate_highest_trajectory, get_all_possible_trajectories, get_steps_for_y_hit, get_xs_for_y, TargetArea, Trajectory};

    #[test]
    fn test_target_area_from_str() {
        assert_eq!("target area: x=50..75, y=20..25".parse(), Ok(TargetArea { x: 50..=75, y: 20..=25 }));
        assert_eq!("target area: x=-50..75, y=20..-25".parse(), Ok(TargetArea { x: -50..=75, y: 20..=-25 }));
        assert_eq!("target area: x=-50..-20, y=-10..-5".parse(), Ok(TargetArea { x: -50..=-20, y: -10..=-5 }));
    }

    #[test]
    fn test_get_top() {
        assert_eq!(Trajectory { x: 6, y: 9 }.get_top(), 45);
    }

    #[test]
    fn test_calculate_highest_trajectory() {
        let target = TargetArea { x: 20..=30, y: -10..=-5 };
        let trajectory = calculate_highest_trajectory(&target);
        assert_eq!(trajectory, Some(Trajectory { x: 6, y: 9 }));
    }

    #[test]
    fn test_get_steps_for_y_hit() {
        let target = TargetArea { x: 20..=30, y: -10..=-5 };
        assert_eq!(get_steps_for_y_hit(&target, 0), Some(4..=5));
        assert_eq!(get_steps_for_y_hit(&target, 5), Some(11..=11));
    }

    #[test]
    fn test_get_xs_for_y() {
        let target = TargetArea { x: 20..=30, y: -10..=-5 };
        assert_eq!(get_xs_for_y(&target, -10), Some(vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]));
        assert_eq!(get_xs_for_y(&target, -2), Some(vec![8, 9, 10, 11, 12, 13, 14, 15]));
    }

    #[test]
    fn test_get_all_trajectories() {
        let target = TargetArea { x: 20..=30, y: -10..=-5 };
        let trajectories = get_all_possible_trajectories(&target).unwrap();

        let expected: Vec<Trajectory> = vec![
            (23, -10).into(), (25, -9).into(), (27, -5).into(), (29, -6).into(), (22, -6).into(), (21, -7).into(), (9, 0).into(), (27, -7).into(), (24, -5).into(),
            (25, -7).into(), (26, -6).into(), (25, -5).into(), (6, 8).into(), (11, -2).into(), (20, -5).into(), (29, -10).into(), (6, 3).into(), (28, -7).into(),
            (8, 0).into(), (30, -6).into(), (29, -8).into(), (20, -10).into(), (6, 7).into(), (6, 4).into(), (6, 1).into(), (14, -4).into(), (21, -6).into(),
            (26, -10).into(), (7, -1).into(), (7, 7).into(), (8, -1).into(), (21, -9).into(), (6, 2).into(), (20, -7).into(), (30, -10).into(), (14, -3).into(),
            (20, -8).into(), (13, -2).into(), (7, 3).into(), (28, -8).into(), (29, -9).into(), (15, -3).into(), (22, -5).into(), (26, -8).into(), (25, -8).into(),
            (25, -6).into(), (15, -4).into(), (9, -2).into(), (15, -2).into(), (12, -2).into(), (28, -9).into(), (12, -3).into(), (24, -6).into(), (23, -7).into(),
            (25, -10).into(), (7, 8).into(), (11, -3).into(), (26, -7).into(), (7, 1).into(), (23, -9).into(), (6, 0).into(), (22, -10).into(), (27, -6).into(),
            (8, 1).into(), (22, -8).into(), (13, -4).into(), (7, 6).into(), (28, -6).into(), (11, -4).into(), (12, -4).into(), (26, -9).into(), (7, 4).into(),
            (24, -10).into(), (23, -8).into(), (30, -8).into(), (7, 0).into(), (9, -1).into(), (10, -1).into(), (26, -5).into(), (22, -9).into(), (6, 5).into(),
            (7, 5).into(), (23, -6).into(), (28, -10).into(), (10, -2).into(), (11, -1).into(), (20, -9).into(), (14, -2).into(), (29, -7).into(), (13, -3).into(),
            (23, -5).into(), (24, -8).into(), (27, -9).into(), (30, -7).into(), (28, -5).into(), (21, -10).into(), (7, 9).into(), (6, 6).into(), (21, -5).into(),
            (27, -10).into(), (7, 2).into(), (30, -9).into(), (21, -8).into(), (22, -7).into(), (24, -9).into(), (20, -6).into(), (6, 9).into(), (29, -5).into(),
            (8, -2).into(), (27, -8).into(), (30, -5).into(), (24, -7).into(),
        ];

        let missing: Vec<Trajectory> = expected.iter().cloned().filter(|t| !trajectories.contains(t)).collect();
        let extra: Vec<Trajectory> = trajectories.iter().cloned().filter(|t| !expected.contains(t)).collect();
        assert_eq!(trajectories.len(), 112);
        assert_eq!(missing, vec![]);
        assert_eq!(extra, vec![]);
    }
}