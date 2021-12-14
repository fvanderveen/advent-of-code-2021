use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use crate::days::Day;

pub const DAY12: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let system: CaveSystem = input.as_str().try_into().unwrap();

    let paths = system.paths(system.start(), &vec![], 1);

    println!("Puzzle 1 answer: {}", paths.len());
}

fn puzzle2(input: &String) {
    let system: CaveSystem = input.as_str().try_into().unwrap();

    let paths = system.paths(system.start(), &vec![], 2);

    println!("Puzzle 2 answer: {}", paths.len());
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum CaveType {
    Big,
    Small,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Cave<'t> {
    name: &'t str,
    ty: CaveType,
}

#[derive(Eq, PartialEq, Debug)]
struct CaveSystem<'t> {
    caves: Vec<Cave<'t>>,
    connections: HashMap<Cave<'t>, Vec<Cave<'t>>>,
}

fn add_connection<'t>(map: &mut HashMap<Cave<'t>, Vec<Cave<'t>>>, from: Cave<'t>, to: Cave<'t>) {
    let mut existing = match map.get(&from) {
        Some(v) if v.contains(&to) => return,
        Some(v) => v.clone(),
        None => vec![]
    };
    existing.push(to);
    map.insert(from, existing);
}

impl<'t> CaveSystem<'t> {
    fn add(&mut self, from: Cave<'t>, to: Cave<'t>) {
        if !self.caves.contains(&from) { self.caves.push(from); }
        if !self.caves.contains(&to) { self.caves.push(to); }

        add_connection(&mut self.connections, from, to);
        add_connection(&mut self.connections, to, from);
    }

    fn start(&self) -> &Cave {
        self.caves.iter().find(|c| c.name == "start").unwrap()
    }

    fn paths(&self, from: &Cave, visited: &Vec<&Cave>, max_visits_small: usize) -> Vec<String> {
        let results = vec![];
        let visit_count = visited.into_iter().filter(|v| v.name == from.name).count();

        if from.name == "start" && visit_count == 1 {
            return results;
        }

        let mut new_max_visits = max_visits_small;
        if from.ty == CaveType::Small {
            if visit_count >= max_visits_small {
                return results;
            }

            if visit_count > 0 {
                new_max_visits -= 1;
            }
        }

        let mut new_visited = Vec::from(visited.clone());
        new_visited.push(from);

        if from.name == "end" {
            let parts: Vec<String> = new_visited.iter().map(|c| c.name.to_owned()).collect();
            return vec![parts.join(",")];
        }


        match self.connections.get(from) {
            None => vec![],
            Some(v) => {
                let set: HashSet<String, RandomState> = HashSet::from_iter(v.iter().flat_map(|v| self.paths(v, &new_visited, new_max_visits)));
                Vec::from_iter(set.iter().cloned())
            }
        }
    }
}

struct Connection<'t> {
    from: Cave<'t>,
    to: Cave<'t>,
}

impl<'t> TryFrom<&'t str> for Cave<'t> {
    type Error = String;

    fn try_from(input: &'t str) -> Result<Self, Self::Error> {
        let is_big = input.chars().all(|c| c.is_ascii_alphabetic() && c.is_uppercase());
        let is_small = input.chars().all(|c| c.is_ascii_alphabetic() && c.is_lowercase());

        if is_big == is_small {
            Err(format!("Mixed case is not allowed for cave: {}", input))
        } else if is_big {
            Ok(Cave { name: input, ty: CaveType::Big })
        } else {
            Ok(Cave { name: input, ty: CaveType::Small })
        }
    }
}

impl<'t> TryFrom<&'t str> for Connection<'t> {
    type Error = String;

    fn try_from(input: &'t str) -> Result<Self, Self::Error> {
        let caves: Result<Vec<Cave>, String> = input.split("-").map(|p| p.try_into()).collect();

        match caves {
            Err(e) => Err(e),
            Ok(v) if v.len() == 2 => Ok(Connection { from: v[0].clone(), to: v[1].clone() }),
            Ok(v) => Err(format!("Expected <cave>-<cave>, but matched {} caves in {}", v.len(), input)),
        }
    }
}

impl<'t> TryFrom<&'t str> for CaveSystem<'t> {
    type Error = String;

    fn try_from(input: &'t str) -> Result<Self, Self::Error> {
        let result: Result<Vec<Connection>, String> = input.lines().map(|l| l.try_into()).collect();
        let raw_connections = match result {
            Err(e) => return Err(e),
            Ok(v) => v
        };

        let mut system = CaveSystem { caves: vec![], connections: HashMap::new() };

        for connection in raw_connections {
            system.add(connection.from, connection.to)
        }

        Ok(system)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day12::{Cave, CaveSystem};
    use crate::days::day12::CaveType::{Big, Small};

    const EXAMPLE_INPUT_SMALL: &str = "\
        start-A\n\
        start-b\n\
        A-c\n\
        A-b\n\
        b-d\n\
        A-end\n\
        b-end\
    ";

    const EXAMPLE_INPUT_LARGE: &str = "\
        dc-end\n\
        HN-start\n\
        start-kj\n\
        dc-start\n\
        dc-HN\n\
        LN-dc\n\
        HN-end\n\
        kj-sa\n\
        kj-HN\n\
        kj-dc\
    ";

    const EXAMPLE_INPUT_HUGE: &str = "\
        fs-end\n\
        he-DX\n\
        fs-he\n\
        start-DX\n\
        pj-DX\n\
        end-zg\n\
        zg-sl\n\
        zg-pj\n\
        pj-he\n\
        RW-he\n\
        fs-DX\n\
        pj-RW\n\
        zg-RW\n\
        start-pj\n\
        he-WI\n\
        zg-he\n\
        pj-fs\n\
        start-RW\
    ";

    fn big(name: &str) -> Cave { Cave { name, ty: Big } }

    fn small(name: &str) -> Cave { Cave { name, ty: Small } }

    #[test]
    fn test_parse() {
        let system: Result<CaveSystem, String> = EXAMPLE_INPUT_SMALL.try_into();
        assert_eq!(system, Ok(CaveSystem {
            caves: vec![small("start"), big("A"), small("b"), small("c"), small("d"), small("end")],
            connections: HashMap::from([
                (small("start"), vec![big("A"), small("b")]),
                (big("A"), vec![small("start"), small("c"), small("b"), small("end")]),
                (small("b"), vec![small("start"), big("A"), small("d"), small("end")]),
                (small("c"), vec![big("A")]),
                (small("d"), vec![small("b")]),
                (small("end"), vec![big("A"), small("b")]),
            ]),
        }));
    }

    #[test]
    fn test_paths_small() {
        let system: CaveSystem = EXAMPLE_INPUT_SMALL.try_into().unwrap();

        let mut paths = system.paths(system.start(), &vec![], 1);
        paths.sort();
        assert_eq!(paths.len(), 10);
        assert_eq!(paths, vec![
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,end",
        ]);

        let mut paths_2 = system.paths(system.start(), &vec![], 2);
        paths_2.sort();
        assert_eq!(paths_2.len(), 36);
        assert_eq!(paths_2, vec![
            "start,A,b,A,b,A,c,A,end",
            "start,A,b,A,b,A,end",
            "start,A,b,A,b,end",
            "start,A,b,A,c,A,b,A,end",
            "start,A,b,A,c,A,b,end",
            "start,A,b,A,c,A,c,A,end",
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,d,b,A,c,A,end",
            "start,A,b,d,b,A,end",
            "start,A,b,d,b,end",
            "start,A,b,end",
            "start,A,c,A,b,A,b,A,end",
            "start,A,c,A,b,A,b,end",
            "start,A,c,A,b,A,c,A,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,d,b,A,end",
            "start,A,c,A,b,d,b,end",
            "start,A,c,A,b,end",
            "start,A,c,A,c,A,b,A,end",
            "start,A,c,A,c,A,b,end",
            "start,A,c,A,c,A,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,b,A,c,A,end",
            "start,b,A,b,A,end",
            "start,b,A,b,end",
            "start,b,A,c,A,b,A,end",
            "start,b,A,c,A,b,end",
            "start,b,A,c,A,c,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,d,b,A,c,A,end",
            "start,b,d,b,A,end",
            "start,b,d,b,end",
            "start,b,end",
        ]);
    }

    #[test]
    fn test_paths_large() {
        let system: CaveSystem = EXAMPLE_INPUT_LARGE.try_into().unwrap();

        let mut paths = system.paths(system.start(), &vec![], 1);
        paths.sort();

        assert_eq!(paths.len(), 19);
        assert_eq!(paths, vec![
            "start,HN,dc,HN,end",
            "start,HN,dc,HN,kj,HN,end",
            "start,HN,dc,end",
            "start,HN,dc,kj,HN,end",
            "start,HN,end",
            "start,HN,kj,HN,dc,HN,end",
            "start,HN,kj,HN,dc,end",
            "start,HN,kj,HN,end",
            "start,HN,kj,dc,HN,end",
            "start,HN,kj,dc,end",
            "start,dc,HN,end",
            "start,dc,HN,kj,HN,end",
            "start,dc,end",
            "start,dc,kj,HN,end",
            "start,kj,HN,dc,HN,end",
            "start,kj,HN,dc,end",
            "start,kj,HN,end",
            "start,kj,dc,HN,end",
            "start,kj,dc,end",
        ]);

        assert_eq!(system.paths(system.start(), &vec![], 2).len(), 103);
    }

    #[test]
    fn test_paths_huge() {
        let system: CaveSystem = EXAMPLE_INPUT_HUGE.try_into().unwrap();

        let paths = system.paths(system.start(), &vec![], 1);
        assert_eq!(paths.len(), 226);
        assert_eq!(system.paths(system.start(), &vec![], 2).len(), 3509);
    }
}