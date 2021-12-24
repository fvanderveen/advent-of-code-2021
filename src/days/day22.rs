use std::ops::RangeInclusive;
use std::str::FromStr;
use regex::Regex;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};
use crate::days::Day;
use crate::util::number;

pub const DAY22: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    println!("Puzzle 1 answer: {}", puzzle.count_initialize(Some(-50..=50)));
}

fn puzzle2(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    println!("Puzzle 2 answer: {}", puzzle.count_initialize(None));
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Cube {
    On,
    Off,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Range3D {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

fn overlap_range(range: &RangeInclusive<isize>, other: &RangeInclusive<isize>) -> RangeInclusive<isize> {
    let range_start = range.start().clone();
    let range_end = range.end().clone();
    let other_start = other.start().clone();
    let other_end = other.end().clone();

    other_start.max(range_start)..=other_end.min(range_end)
}

impl Range3D {
    fn capped_single(&self, cap: &RangeInclusive<isize>) -> Self {
        let min_x = self.x.start().max(cap.start()).clone();
        let max_x = self.x.end().min(cap.end()).clone();
        let min_y = self.y.start().max(cap.start()).clone();
        let max_y = self.y.end().min(cap.end()).clone();
        let min_z = self.z.start().max(cap.start()).clone();
        let max_z = self.z.end().min(cap.end()).clone();

        Self {
            x: min_x..=max_x,
            y: min_y..=max_y,
            z: min_z..=max_z,
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn volume(&self) -> BigUint {
        let xs = BigInt::from(self.x.start().clone());
        let xe = BigInt::from(self.x.end().clone());
        let ys = BigInt::from(self.y.start().clone());
        let ye = BigInt::from(self.y.end().clone());
        let zs = BigInt::from(self.z.start().clone());
        let ze = BigInt::from(self.z.end().clone());

        // println!("{},{}", xe.clone()-xs.clone(), self.x.clone().count());
        let vol = ((xe-xs+BigInt::one()) * (ye-ys+BigInt::one()) * (ze-zs+BigInt::one())).abs();
        // println!("{}", vol);
        BigUint::try_from(vol).unwrap()
    }

    fn overlap(&self, other: &Range3D) -> Range3D {
        Range3D {
            x: overlap_range(&self.x, &other.x),
            y: overlap_range(&self.y, &other.y),
            z: overlap_range(&self.z, &other.z),
        }
    }
}

impl FromStr for Range3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new("^x=(?P<xmin>-?\\d+)\\.\\.(?P<xmax>-?\\d+),y=(?P<ymin>-?\\d+)\\.\\.(?P<ymax>-?\\d+),z=(?P<zmin>-?\\d+)\\.\\.(?P<zmax>-?\\d+)$").map_err(|e| format!("{}", e))?;

        let captures = regex.captures(s).ok_or(format!("Could not match ranges"))?;
        let xmin = number::parse_isize(captures.name("xmin").ok_or(format!("Missing xmin"))?.as_str())?;
        let xmax = number::parse_isize(captures.name("xmax").ok_or(format!("Missing xmax"))?.as_str())?;
        let ymin = number::parse_isize(captures.name("ymin").ok_or(format!("Missing ymin"))?.as_str())?;
        let ymax = number::parse_isize(captures.name("ymax").ok_or(format!("Missing ymax"))?.as_str())?;
        let zmin = number::parse_isize(captures.name("zmin").ok_or(format!("Missing zmin"))?.as_str())?;
        let zmax = number::parse_isize(captures.name("zmax").ok_or(format!("Missing zmax"))?.as_str())?;

        Ok(Range3D {
            x: xmin..=xmax,
            y: ymin..=ymax,
            z: zmin..=zmax,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Command {
    target: Cube,
    range: Range3D,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: [&str; 2] = s.split(" ").collect::<Vec<_>>().try_into().map_err(|e: Vec<_>| format!("Wrong number of parts {:?}", e))?;
        let target = match parts[0] {
            "on" => Cube::On,
            "off" => Cube::Off,
            _ => return Err(format!("Invalid command '{}'", parts[0]))
        };
        let range: Range3D = parts[1].parse()?;

        Ok(Command { target, range })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Puzzle {
    commands: Vec<Command>,
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let commands: Vec<Command> = s.lines().map(|l| l.parse()).collect::<Result<Vec<_>, String>>()?;

        Ok(Puzzle {
            commands,
        })
    }
}

impl Puzzle {
    fn count_initialize(&self, cap: Option<RangeInclusive<isize>>) -> BigUint {
        // Build a stack, by adding:
        // For each command (t):
        // - For any existing element (u) on the stack, a counter volume for the overlap
        //   - if u == on, add overlap as off (if t = off, this is the bit of u now off. if t = on, this is the bit otherwise double counted)
        //   - it u == off, add overlap as on (if t = on, this is the bit of u now on again, if t = off this is the bit otherwise double subtracted)
        // - The command, if it's turning on.

        let mut stack: Vec<Command> = vec![];
        for command in &self.commands {
            let init_range = cap.clone().map(|c| command.range.capped_single(&c)).unwrap_or(command.range.clone());
            if init_range.is_empty() { continue; }

            for item in &stack.clone() {
                let overlap = init_range.overlap(&item.range);
                if !overlap.is_empty() {
                    stack.push(Command { target: if item.target == Cube::On { Cube::Off } else { Cube::On }, range: overlap });
                }
            }

            if command.target == Cube::On {
                stack.push(Command { target: Cube::On, range: init_range });
            }
        }

        let mut result = BigUint::zero();
        for item in &stack {
            if item.target == Cube::On {
                result += item.range.volume();
            } else {
                result -= item.range.volume()
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{RangeInclusive};
    use std::str::FromStr;
    use num_bigint::BigUint;
    use crate::days::day22::{Command, Cube, overlap_range, Puzzle, Range3D};

    const EXAMPLE_INPUT: &str = "\
        on x=-20..26,y=-36..17,z=-47..7\n\
        on x=-20..33,y=-21..23,z=-26..28\n\
        on x=-22..28,y=-29..23,z=-38..16\n\
        on x=-46..7,y=-6..46,z=-50..-1\n\
        on x=-49..1,y=-3..46,z=-24..28\n\
        on x=2..47,y=-22..22,z=-23..27\n\
        on x=-27..23,y=-28..26,z=-21..29\n\
        on x=-39..5,y=-6..47,z=-3..44\n\
        on x=-30..21,y=-8..43,z=-13..34\n\
        on x=-22..26,y=-27..20,z=-29..19\n\
        off x=-48..-32,y=26..41,z=-47..-37\n\
        on x=-12..35,y=6..50,z=-50..-2\n\
        off x=-48..-32,y=-32..-16,z=-15..-5\n\
        on x=-18..26,y=-33..15,z=-7..46\n\
        off x=-40..-22,y=-38..-28,z=23..41\n\
        on x=-16..35,y=-41..10,z=-47..6\n\
        off x=-32..-23,y=11..30,z=-14..3\n\
        on x=-49..-5,y=-3..45,z=-29..18\n\
        off x=18..30,y=-20..-8,z=-3..13\n\
        on x=-41..9,y=-7..43,z=-33..15\n\
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
        on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test_parse_puzzle() {
        let puzzle: Result<Puzzle, String> = EXAMPLE_INPUT.parse();

        fn on(x: RangeInclusive<isize>, y: RangeInclusive<isize>, z: RangeInclusive<isize>) -> Command {
            Command { target: Cube::On, range: Range3D { x, y, z } }
        }
        fn off(x: RangeInclusive<isize>, y: RangeInclusive<isize>, z: RangeInclusive<isize>) -> Command {
            Command { target: Cube::Off, range: Range3D { x, y, z } }
        }

        assert_eq!(puzzle, Ok(Puzzle {
            commands: vec![
                on(-20..=26, -36..=17, -47..=7),
                on(-20..=33, -21..=23, -26..=28),
                on(-22..=28, -29..=23, -38..=16),
                on(-46..=7, -6..=46, -50..=-1),
                on(-49..=1, -3..=46, -24..=28),
                on(2..=47, -22..=22, -23..=27),
                on(-27..=23, -28..=26, -21..=29),
                on(-39..=5, -6..=47, -3..=44),
                on(-30..=21, -8..=43, -13..=34),
                on(-22..=26, -27..=20, -29..=19),
                off(-48..=-32, 26..=41, -47..=-37),
                on(-12..=35, 6..=50, -50..=-2),
                off(-48..=-32, -32..=-16, -15..=-5),
                on(-18..=26, -33..=15, -7..=46),
                off(-40..=-22, -38..=-28, 23..=41),
                on(-16..=35, -41..=10, -47..=6),
                off(-32..=-23, 11..=30, -14..=3),
                on(-49..=-5, -3..=45, -29..=18),
                off(18..=30, -20..=-8, -3..=13),
                on(-41..=9, -7..=43, -33..=15),
                on(-54112..=-39298, -85059..=-49293, -27449..=7877),
                on(967..=23432, 45373..=81175, 27513..=53682),
            ],
        }))
    }

    #[test]
    fn test_capped() {
        let range = Range3D {
            x: -100..=100,
            y: -50..=50,
            z: -24..=24,
        };

        assert_eq!(range.capped_single(&(-50..=50)), Range3D { x: -50..=50, y: -50..=50, z: -24..=24 });
        assert_eq!(range.capped_single(&(-50..=50)).is_empty(), false);
        assert_eq!(range.capped_single(&(-20..=20)), Range3D { x: -20..=20, y: -20..=20, z: -20..=20 });
        assert_eq!(range.capped_single(&(-20..=20)).is_empty(), false);
        assert_eq!(range.capped_single(&(50..=100)), Range3D { x: 50..=100, y: 50..=50, z: 50..=24 });
        assert_eq!(range.capped_single(&(50..=100)).is_empty(), true);
    }

    #[test]
    fn test_initialize() {
        let mut puzzle = Puzzle {
            commands: vec![
                Command { target: Cube::On, range: Range3D { x: 0..=20, y: 0..=20, z: 0..=20 } }
            ]
        };
        assert_eq!(puzzle.count_initialize(None), BigUint::from(21 * 21 * 21 as usize));
        puzzle.commands.push(Command { target: Cube::On, range: Range3D { x: 10..=30, y: 10..=30, z: 10..=30 } });
        assert_eq!(puzzle.count_initialize(None), BigUint::from(21 * 21 * 21 + 21 * 21 * 21 - (11 * 11 * 11) as usize));

        puzzle.commands.push(Command { target: Cube::Off, range: Range3D { x: 0..=5, y: -10..=5, z: -20..=5 } });
        assert_eq!(puzzle.count_initialize(None), BigUint::from(21 * 21 * 21 + 21 * 21 * 21 - (11 * 11 * 11) - (6 * 6 * 6) as usize));
    }

    #[test]
    fn test_initialize_example() {
        let puzzle: Puzzle = EXAMPLE_INPUT.parse().unwrap();

        assert_eq!(puzzle.count_initialize(Some(-50..=50)), BigUint::from(590784 as usize));
        let v0 = BigUint::from(590784 as usize);
        let v1 = BigUint::from_str("18719357085335").unwrap();
        let v2 = BigUint::from_str("21049844681660").unwrap();

        assert_eq!(puzzle.count_initialize(None), v0 + v1 + v2);

        let puzzle2: Puzzle = EXAMPLE_INPUT_2.parse().unwrap();
        assert_eq!(puzzle2.count_initialize(None), BigUint::from_str("2758514936282235").unwrap());
    }

    #[test]
    fn test_overlap_range() {
        // no overlap
        assert_eq!(overlap_range(&(10..=20), &(30..=40)), 30..=20);
        // overlap at start
        assert_eq!(overlap_range(&(10..=20), &(5..=15)), 10..=15);
        // overlap in middle
        assert_eq!(overlap_range(&(10..=20), &(13..=17)), 13..=17);
        // overlap at end
        assert_eq!(overlap_range(&(10..=20), &(15..=30)), 15..=20);
        // full overlap
        assert_eq!(overlap_range(&(10..=20), &(5..=30)), 10..=20);
    }

    #[test]
    fn test_volume() {
        assert_eq!(Range3D { x: -54112..=-39298, y: -85059..=-49293, z: -27449..=7877 }.volume(), BigUint::from_str("18719357085335").unwrap());
        assert_eq!(Range3D { x: 967..=23432, y: 45373..=81175, z: 27513..=53682 }.volume(), BigUint::from_str("21049844681660").unwrap());
    }

    const EXAMPLE_INPUT_2: &str = "\
        on x=-5..47,y=-31..22,z=-19..33\n\
        on x=-44..5,y=-27..21,z=-14..35\n\
        on x=-49..-1,y=-11..42,z=-10..38\n\
        on x=-20..34,y=-40..6,z=-44..1\n\
        off x=26..39,y=40..50,z=-2..11\n\
        on x=-41..5,y=-41..6,z=-36..8\n\
        off x=-43..-33,y=-45..-28,z=7..25\n\
        on x=-33..15,y=-32..19,z=-34..11\n\
        off x=35..47,y=-46..-34,z=-11..5\n\
        on x=-14..36,y=-6..44,z=-16..29\n\
        on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
        on x=36731..105352,y=-21140..28532,z=16094..90401\n\
        on x=30999..107136,y=-53464..15513,z=8553..71215\n\
        on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
        on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
        on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
        on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
        on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
        on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
        on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
        on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
        on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
        on x=-726..56291,y=-62629..13224,z=18033..85226\n\
        on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
        on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
        on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
        on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
        on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
        on x=-57817..18248,y=49321..117703,z=5745..55881\n\
        on x=14781..98692,y=-1341..70827,z=15753..70151\n\
        on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
        on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
        on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
        on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
        on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
        on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
        on x=16383..101554,y=4615..83635,z=-44907..18747\n\
        off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
        on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
        on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
        on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
        on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
        off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
        off x=2032..69770,y=-71013..4824,z=7471..94418\n\
        on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
        off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
        off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
        off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
        on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
        off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
        off x=18020..114017,y=-48931..32606,z=21474..89843\n\
        off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
        off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
        on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
        off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
        off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
        off x=-27365..46395,y=31009..98017,z=15428..76570\n\
        off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
        on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
        off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
}