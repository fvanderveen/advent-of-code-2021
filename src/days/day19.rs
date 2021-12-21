use std::cmp::Ordering;
use std::str::FromStr;
use regex::Regex;
use crate::days::Day;
use crate::days::day19::FacingDirection::{XNeg, XPos, YNeg, YPos, ZNeg, ZPos};
use crate::util::collection::CollectionExtension;
use crate::util::number::parse_isize;

pub const DAY19: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let scanners = parse_input(input).unwrap();
    let beacons = map_all_beacons(&scanners);

    println!("Puzzle 1 answer: {}", beacons.len());
}

fn puzzle2(input: &String) {
    let scanners = parse_input(input).unwrap();

    let mapped = map_scanners(&scanners);

    let mut max_manhattan = 0;
    for i in 0..mapped.len() {
        let current = &mapped[i];
        if let Some(max) = mapped.iter().skip(i)
            .map(|other| current.location.manhattan(&other.location))
            .max() {
            max_manhattan = max_manhattan.max(max);
        }
    }

    println!("Puzzle 2 answer: {}", max_manhattan);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.split(",").map(|p| parse_isize(p)).collect::<Result<Vec<isize>, String>>()?;
        if points.len() != 3 {
            Err(format!("Expected three coordinates, but got {}", points.len()))
        } else {
            Ok(Point3D { x: points[0], y: points[1], z: points[2] })
        }
    }
}

impl PartialOrd<Self> for Point3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.z.cmp(&other.z))
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum FacingDirection { XPos, XNeg, YPos, YNeg, ZPos, ZNeg }

impl Point3D {
    fn empty() -> Self {
        Point3D { x: 0, y: 0, z: 0 }
    }

    fn distance(&self, other: &Self) -> Self {
        Point3D {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    fn manhattan(&self, other: &Self) -> usize {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();
        let z = (self.z - other.z).abs();
        return (x + y + z) as usize;
    }

    fn translate(&self, other: &Self) -> Self {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn rotate(&self, facing: FacingDirection, rotation: usize) -> Point3D {
        let turned = match facing {
            FacingDirection::XPos => self.clone(),
            FacingDirection::XNeg => Point3D { x: -self.x, y: -self.y, z: self.z },
            FacingDirection::YPos => Point3D { x: self.y, y: -self.x, z: self.z },
            FacingDirection::YNeg => Point3D { x: -self.y, y: self.x, z: self.z },
            FacingDirection::ZPos => Point3D { x: self.z, y: self.y, z: -self.x },
            FacingDirection::ZNeg => Point3D { x: -self.z, y: self.y, z: self.x },
        };

        match rotation {
            0 => turned,
            90 => Point3D { x: turned.x, y: -turned.z, z: turned.y },
            180 => Point3D { x: turned.x, y: -turned.y, z: -turned.z },
            270 => Point3D { x: turned.x, y: turned.z, z: -turned.y },
            _ => panic!("Invalid rotation {}", rotation)
        }
    }
}

fn get_relative_distances(point: &Point3D, points: &Vec<Point3D>) -> Vec<Point3D> {
    points.iter().map(|p| point.distance(p)).collect()
}


fn get_manhattan_distances(point: &Point3D, points: &Vec<Point3D>) -> Vec<usize> {
    points.iter().map(|p| point.manhattan(p)).collect()
}

fn get_overlapping_values(left: &Vec<Point3D>, right: &Vec<Point3D>) -> Vec<Point3D> {
    left.iter().cloned().filter(|v| right.contains(v)).collect()
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Scanner {
    name: String,
    location: Point3D,
    points: Vec<Point3D>,
}

impl Scanner {
    fn translate(&self, by: &Point3D) -> Scanner {
        Scanner {
            name: self.name.clone(),
            location: self.location.translate(by),
            points: self.points.iter().map(|p| p.translate(by)).collect(),
        }
    }

    fn rotate(&self, facing: FacingDirection, rotation: usize) -> Self {
        Scanner {
            name: self.name.clone(),
            location: self.location.rotate(facing, rotation),
            points: self.points.iter().map(|p| p.rotate(facing, rotation)).collect(),
        }
    }
}

fn parse_scanner(input: &str) -> Result<Scanner, String> {
    // Skip the first line, with the `--- scanner # ---`
    let lines: Vec<&str> = input.lines().collect();
    let regex = Regex::new("^--- scanner (.*) ---$").map_err(|e| format!("{}", e))?;
    let name = regex.captures(lines[0]).and_then(|c| c.get(0)).map(|m| m.as_str().to_owned()).ok_or("No name found".to_owned())?;

    let points: Vec<Point3D> = lines.iter().skip(1).map(|l| l.parse()).collect::<Result<Vec<Point3D>, String>>()?;
    Ok(Scanner { name, location: Point3D::empty(), points })
}

fn parse_input(input: &str) -> Result<Vec<Scanner>, String> {
    input.split("\n\n").map(|s| parse_scanner(s)).collect()
}

fn find_match(scanner: &Scanner, others: &Vec<Scanner>) -> Option<Scanner> {
    for scanner_index in 0..scanner.points.len() {
        let manhattan_distances = get_manhattan_distances(&scanner.points[scanner_index], &scanner.points);
        for other in others {
            for other_index in 0..other.points.len() {
                // Only consider going through the options if there are 12 matching manhattan distances (to avoid computing rotations when not even needed).
                let other_manhattan_distances = get_manhattan_distances(&other.points[other_index], &other.points);
                let union = other_manhattan_distances.union(&manhattan_distances);
                if union.len() < 12 {
                    continue;
                }

                // There is a chance.
                let scanner_distances = get_relative_distances(&scanner.points[scanner_index], &scanner.points);
                for direction in [XPos, XNeg, YPos, YNeg, ZPos, ZNeg] {
                    for rotation in [0, 90, 180, 270] {
                        let rotated: Vec<_> = other.points.iter().map(|p| p.rotate(direction, rotation)).collect();
                        let rotated_distances = get_relative_distances(&rotated[other_index], &rotated);
                        let overlap = get_overlapping_values(&scanner_distances, &rotated_distances);
                        if overlap.len() >= 12 {
                            // Should be a pair? Try determine relative position of this scanner.
                            // The 0-point values should give a distance that should match the new position:
                            let translation = &rotated[other_index].distance(&scanner.points[scanner_index]);
                            return Some(Scanner { name: other.name.clone(), location: translation.clone(), points: rotated.iter().map(|p| p.translate(translation)).collect() });
                        }
                    }
                }
            }
        }
    }

    None
}

fn map_all_beacons(scanners: &Vec<Scanner>) -> Vec<Point3D> {
    let beacons: Vec<_> = map_scanners(scanners).iter().flat_map(|s| s.points.clone()).collect();
    beacons.deduplicate()
}

fn map_scanners(scanners: &Vec<Scanner>) -> Vec<Scanner> {
    // The first scanner will be the anchor. let's match others, somehow.
    let main = scanners[0].clone();

    let mut mapped = vec![scanners[0].clone()];
    let mut to_map: Vec<_> = scanners.iter().cloned().filter(|s| main.ne(s)).collect();

    'main: while !to_map.is_empty() {
        let mut revmapped = mapped.clone();
        revmapped.reverse();
        for mapped_scanner in &revmapped {
            if let Some(matched) = find_match(mapped_scanner, &to_map) {
                to_map.retain(|v| v.name != matched.name);
                mapped.push(matched);
                continue 'main;
            }
        }

        panic!("Did not match any new scanner location 😱");
    }

    mapped
}

#[cfg(test)]
mod tests {
    use crate::days::day19::{find_match, map_all_beacons, parse_input, Point3D};
    use crate::days::day19::FacingDirection::{XNeg, YNeg};

    #[test]
    fn test_find_match() {
        let scanners = parse_input(EXAMPLE_INPUT).unwrap();

        let first = &scanners[0];
        let others = scanners.iter().cloned().filter(|s| first.ne(s)).collect();
        let first_match = find_match(first, &others);

        let first_expected = scanners[1].clone().rotate(XNeg, 180).translate(&Point3D { x: 68, y: -1246, z: -43 });
        assert_eq!(first_match, Some(first_expected));

        let second_expected = scanners[4].rotate(YNeg, 90).translate(&Point3D { x: -20, y: -1133, z: 1061 });
        let second_match = find_match(&first_match.unwrap(), &others.iter().cloned().filter(|s| s.ne(&scanners[1])).collect());
        assert_eq!(second_match, Some(second_expected));
    }

    #[test]
    fn test_map_all_beacons() {
        let scanners = parse_input(EXAMPLE_INPUT).unwrap();
        let mut beacons = map_all_beacons(&scanners);
        beacons.sort();

        let mut expected = EXAMPLE_BEACONS.lines().map(|l| l.parse()).collect::<Result<Vec<Point3D>, String>>().unwrap();
        expected.sort();

        assert_eq!(beacons.len(), 79);
        assert_eq!(beacons, expected);
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(Point3D { x: 1105, y: -1205, z: 1229 }.manhattan(&Point3D { x: -92, y: -2380, z: -20 }), 3621);
        assert_eq!(Point3D { x: -92, y: -2380, z: -20 }.manhattan(&Point3D { x: 1105, y: -1205, z: 1229 }), 3621);
    }

    const EXAMPLE_INPUT: &str = "\
        --- scanner 0 ---\n\
        404,-588,-901\n\
        528,-643,409\n\
        -838,591,734\n\
        390,-675,-793\n\
        -537,-823,-458\n\
        -485,-357,347\n\
        -345,-311,381\n\
        -661,-816,-575\n\
        -876,649,763\n\
        -618,-824,-621\n\
        553,345,-567\n\
        474,580,667\n\
        -447,-329,318\n\
        -584,868,-557\n\
        544,-627,-890\n\
        564,392,-477\n\
        455,729,728\n\
        -892,524,684\n\
        -689,845,-530\n\
        423,-701,434\n\
        7,-33,-71\n\
        630,319,-379\n\
        443,580,662\n\
        -789,900,-551\n\
        459,-707,401\n\
        \n\
        --- scanner 1 ---\n\
        686,422,578\n\
        605,423,415\n\
        515,917,-361\n\
        -336,658,858\n\
        95,138,22\n\
        -476,619,847\n\
        -340,-569,-846\n\
        567,-361,727\n\
        -460,603,-452\n\
        669,-402,600\n\
        729,430,532\n\
        -500,-761,534\n\
        -322,571,750\n\
        -466,-666,-811\n\
        -429,-592,574\n\
        -355,545,-477\n\
        703,-491,-529\n\
        -328,-685,520\n\
        413,935,-424\n\
        -391,539,-444\n\
        586,-435,557\n\
        -364,-763,-893\n\
        807,-499,-711\n\
        755,-354,-619\n\
        553,889,-390\n\
        \n\
        --- scanner 2 ---\n\
        649,640,665\n\
        682,-795,504\n\
        -784,533,-524\n\
        -644,584,-595\n\
        -588,-843,648\n\
        -30,6,44\n\
        -674,560,763\n\
        500,723,-460\n\
        609,671,-379\n\
        -555,-800,653\n\
        -675,-892,-343\n\
        697,-426,-610\n\
        578,704,681\n\
        493,664,-388\n\
        -671,-858,530\n\
        -667,343,800\n\
        571,-461,-707\n\
        -138,-166,112\n\
        -889,563,-600\n\
        646,-828,498\n\
        640,759,510\n\
        -630,509,768\n\
        -681,-892,-333\n\
        673,-379,-804\n\
        -742,-814,-386\n\
        577,-820,562\n\
        \n\
        --- scanner 3 ---\n\
        -589,542,597\n\
        605,-692,669\n\
        -500,565,-823\n\
        -660,373,557\n\
        -458,-679,-417\n\
        -488,449,543\n\
        -626,468,-788\n\
        338,-750,-386\n\
        528,-832,-391\n\
        562,-778,733\n\
        -938,-730,414\n\
        543,643,-506\n\
        -524,371,-870\n\
        407,773,750\n\
        -104,29,83\n\
        378,-903,-323\n\
        -778,-728,485\n\
        426,699,580\n\
        -438,-605,-362\n\
        -469,-447,-387\n\
        509,732,623\n\
        647,635,-688\n\
        -868,-804,481\n\
        614,-800,639\n\
        595,780,-596\n\
        \n\
        --- scanner 4 ---\n\
        727,592,562\n\
        -293,-554,779\n\
        441,611,-461\n\
        -714,465,-776\n\
        -743,427,-804\n\
        -660,-479,-426\n\
        832,-632,460\n\
        927,-485,-438\n\
        408,393,-506\n\
        466,436,-512\n\
        110,16,151\n\
        -258,-428,682\n\
        -393,719,612\n\
        -211,-452,876\n\
        808,-476,-593\n\
        -575,615,604\n\
        -485,667,467\n\
        -680,325,-822\n\
        -627,-443,-432\n\
        872,-547,-609\n\
        833,512,582\n\
        807,604,487\n\
        839,-516,451\n\
        891,-625,532\n\
        -652,-548,-490\n\
        30,-46,-14";

    const EXAMPLE_BEACONS: &str = "\
        -892,524,684\n\
        -876,649,763\n\
        -838,591,734\n\
        -789,900,-551\n\
        -739,-1745,668\n\
        -706,-3180,-659\n\
        -697,-3072,-689\n\
        -689,845,-530\n\
        -687,-1600,576\n\
        -661,-816,-575\n\
        -654,-3158,-753\n\
        -635,-1737,486\n\
        -631,-672,1502\n\
        -624,-1620,1868\n\
        -620,-3212,371\n\
        -618,-824,-621\n\
        -612,-1695,1788\n\
        -601,-1648,-643\n\
        -584,868,-557\n\
        -537,-823,-458\n\
        -532,-1715,1894\n\
        -518,-1681,-600\n\
        -499,-1607,-770\n\
        -485,-357,347\n\
        -470,-3283,303\n\
        -456,-621,1527\n\
        -447,-329,318\n\
        -430,-3130,366\n\
        -413,-627,1469\n\
        -345,-311,381\n\
        -36,-1284,1171\n\
        -27,-1108,-65\n\
        7,-33,-71\n\
        12,-2351,-103\n\
        26,-1119,1091\n\
        346,-2985,342\n\
        366,-3059,397\n\
        377,-2827,367\n\
        390,-675,-793\n\
        396,-1931,-563\n\
        404,-588,-901\n\
        408,-1815,803\n\
        423,-701,434\n\
        432,-2009,850\n\
        443,580,662\n\
        455,729,728\n\
        456,-540,1869\n\
        459,-707,401\n\
        465,-695,1988\n\
        474,580,667\n\
        496,-1584,1900\n\
        497,-1838,-617\n\
        527,-524,1933\n\
        528,-643,409\n\
        534,-1912,768\n\
        544,-627,-890\n\
        553,345,-567\n\
        564,392,-477\n\
        568,-2007,-577\n\
        605,-1665,1952\n\
        612,-1593,1893\n\
        630,319,-379\n\
        686,-3108,-505\n\
        776,-3184,-501\n\
        846,-3110,-434\n\
        1135,-1161,1235\n\
        1243,-1093,1063\n\
        1660,-552,429\n\
        1693,-557,386\n\
        1735,-437,1738\n\
        1749,-1800,1813\n\
        1772,-405,1572\n\
        1776,-675,371\n\
        1779,-442,1789\n\
        1780,-1548,337\n\
        1786,-1538,337\n\
        1847,-1591,415\n\
        1889,-1729,1762\n\
        1994,-1805,1792";
}