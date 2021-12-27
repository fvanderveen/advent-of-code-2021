use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use regex::Regex;
use crate::days::Day;

pub const DAY23: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let state: State = input.parse().unwrap();
    
    let result = find_least_energy_sort(&state);
    
    println!("Puzzle 1 answer: {}", result.unwrap().used_energy);
}

fn puzzle2(input: &String) {
    let mut state: State = input.parse().unwrap();

    // Run modifications according to puzzle 2
    state.room_size = 4;
    state.room_a[3] = state.room_a[1];
    state.room_a[1] = Some(Amphipod::D);
    state.room_a[2] = Some(Amphipod::D);
    state.room_b[3] = state.room_b[1];
    state.room_b[1] = Some(Amphipod::C);
    state.room_b[2] = Some(Amphipod::B);
    state.room_c[3] = state.room_c[1];
    state.room_c[1] = Some(Amphipod::B);
    state.room_c[2] = Some(Amphipod::A);
    state.room_d[3] = state.room_d[1];
    state.room_d[1] = Some(Amphipod::A);
    state.room_d[2] = Some(Amphipod::C);

    let result = find_least_energy_sort(&state);

    println!("Puzzle 2 answer: {}", result.unwrap().used_energy);
}

/*
 The map looks like this:
    #############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########
 
 The amphipods each move at most twice:
 1 - Go from a wrong room into the main hallway (any spot not right in front of a room)
 2 - Go from the main hallway into the right room
     Note: this only happens if the room is empty or occupied by the right type.
     
 End state:
    #############
    #...........#
    ###A#B#C#D###
      #A#B#C#D#
      #########
 
 Each step costs:
 A - 1 pt
 B - 10 pt
 C - 100 pt
 D - 1000 pt
 
 Can we use shortest path?
*/

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl FromStr for Amphipod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::A),
            "B" => Ok(Amphipod::B),
            "C" => Ok(Amphipod::C),
            "D" => Ok(Amphipod::D),
            _ => Err(format!("Invalid amphipod: {}", s))
        }
    }
}

impl Amphipod {
    fn get_cost_between(&self, room: Amphipod, room_idx: usize, hallway_idx: usize) -> usize {
        // 0,1,A,2,B,3,C,4,D,5,6
        let steps_to_hallway = room_idx + 1;
        let real_hallway_index = match hallway_idx {
            0 | 1 => hallway_idx,
            2 => 3,
            3 => 5,
            4 => 7,
            5 | 6 => hallway_idx + 4,
            _ => panic!("Invalid hallway index {}", hallway_idx)
        };
        let room_hallway_index = match room {
            Amphipod::A => 2,
            Amphipod::B => 4,
            Amphipod::C => 6,
            Amphipod::D => 8,
        };
        let steps_in_hallway = room_hallway_index.max(real_hallway_index) - real_hallway_index.min(room_hallway_index);
        let total_steps = steps_to_hallway + steps_in_hallway;
        
        match self {
            Amphipod::A => total_steps,
            Amphipod::B => total_steps * 10,
            Amphipod::C => total_steps * 100,
            Amphipod::D => total_steps * 1000
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Default)]
struct State {
    hallway: [Option<Amphipod>; 7],
    room_a: [Option<Amphipod>; 4],
    room_b: [Option<Amphipod>; 4],
    room_c: [Option<Amphipod>; 4],
    room_d: [Option<Amphipod>; 4],
    room_size: usize,
    used_energy: usize,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        if lines.len() != 5 ||
            lines[0].trim().ne("#############") ||
            lines[1].trim().ne("#...........#") ||
            lines[4].trim_end().ne("  #########") {
            return Err(String::from("Invalid format"));
        }

        let line2regex = Regex::new("^###(?P<A>[A-D])#(?P<B>[A-D])#(?P<C>[A-D])#(?P<D>[A-D])###\\s*$").map_err(|e| format!("{}", e))?;
        let line3regex = Regex::new("^  #(?P<A>[A-D])#(?P<B>[A-D])#(?P<C>[A-D])#(?P<D>[A-D])#\\s*$").map_err(|e| format!("{}", e))?;

        fn get_amphipods(regex: Regex, line: &str) -> Result<(Amphipod, Amphipod, Amphipod, Amphipod), String> {
            let line_caps = regex.captures(line).ok_or(format!("Could not match line '{}' with '{}'", line, regex))?;
            let a: Amphipod = line_caps.name("A").ok_or(String::from("Missing <A>"))?.as_str().parse()?;
            let b: Amphipod = line_caps.name("B").ok_or(String::from("Missing <B>"))?.as_str().parse()?;
            let c: Amphipod = line_caps.name("C").ok_or(String::from("Missing <C>"))?.as_str().parse()?;
            let d: Amphipod = line_caps.name("D").ok_or(String::from("Missing <D>"))?.as_str().parse()?;
            Ok((a, b, c, d))
        }

        let (a1, b1, c1, d1) = get_amphipods(line2regex, lines[2])?;
        let (a2, b2, c2, d2) = get_amphipods(line3regex, lines[3])?;

        Ok(State {
            room_a: [Some(a1), Some(a2), None, None],
            room_b: [Some(b1), Some(b2), None, None],
            room_c: [Some(c1), Some(c2), None, None],
            room_d: [Some(d1), Some(d2), None, None],
            room_size: 2,
            ..State::default()
        })
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn to_string(cell: &Option<Amphipod>) -> &str {
            match cell {
                None => ".",
                Some(Amphipod::A) => "A",
                Some(Amphipod::B) => "B",
                Some(Amphipod::C) => "C",
                Some(Amphipod::D) => "D",
            }
        }

        writeln!(f, "#############")?;
        writeln!(f, "#{}{}.{}.{}.{}.{}{}#",
                 to_string(&self.hallway[0]), to_string(&self.hallway[1]),
                 to_string(&self.hallway[2]),
                 to_string(&self.hallway[3]),
                 to_string(&self.hallway[4]),
                 to_string(&self.hallway[5]), to_string(&self.hallway[6]),
        )?;
        writeln!(f, "###{}#{}#{}#{}###",
                 to_string(&self.room_a[0]), to_string(&self.room_b[0]),
                 to_string(&self.room_c[0]), to_string(&self.room_d[0])
        )?;
        for ridx in 1..self.room_size {
            writeln!(f, "  #{}#{}#{}#{}#",
                     to_string(&self.room_a[ridx]), to_string(&self.room_b[ridx]),
                     to_string(&self.room_c[ridx]), to_string(&self.room_d[ridx])
            )?;
        }
        write!(f, "  #########")
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.used_energy.cmp(&self.used_energy)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn get_room_state(&self, room: Amphipod) -> [Option<Amphipod>; 4] {
        match room {
            Amphipod::A => self.room_a,
            Amphipod::B => self.room_b,
            Amphipod::C => self.room_c,
            Amphipod::D => self.room_d,
        }
    }
    
    fn get_room_state_mut(&mut self, room: Amphipod) -> &mut [Option<Amphipod>; 4] {
        match room {
            Amphipod::A => &mut self.room_a,
            Amphipod::B => &mut self.room_b,
            Amphipod::C => &mut self.room_c,
            Amphipod::D => &mut self.room_d,
        }
    }
    
    fn is_finished(&self) -> bool {
        self.hallway.iter().all(|h| h.is_none()) &&
            self.room_a.iter().take(self.room_size).all(|a| Some(Amphipod::A).eq(a)) &&
            self.room_b.iter().take(self.room_size).all(|b| Some(Amphipod::B).eq(b)) &&
            self.room_c.iter().take(self.room_size).all(|c| Some(Amphipod::C).eq(c)) &&
            self.room_d.iter().take(self.room_size).all(|d| Some(Amphipod::D).eq(d))
    }
}

fn get_hallway_index(room: Amphipod) -> (usize, usize) {
    match room {
        Amphipod::A => (1, 2),
        Amphipod::B => (2, 3),
        Amphipod::C => (3, 4),
        Amphipod::D => (4, 5),
    }
}

fn get_hallway_options(room: Amphipod, state: &[Option<Amphipod>; 7]) -> Vec<usize> {
    let (left, right) = get_hallway_index(room);

    let mut result = vec![];

    for idx in (0..=left).rev() {
        if let Some(_) = state[idx] {
            break;
        }
        result.push(idx)
    }
    for idx in right..7 {
        if let Some(_) = state[idx] {
            break;
        }
        result.push(idx)
    }

    result
}

fn can_move_into_room(state: &State, index: usize) -> bool {
    let target = match state.hallway[index] {
        None => return false,
        Some(pod) => pod
    };

    // Check if we can actually move to an index next to the room
    let (left, right) = get_hallway_index(target);
    if index < left {
        if !state.hallway[index+1..=left].iter().all(|h| h.is_none()) {
            return false;
        }
    } else if right < index {
        if !state.hallway[right..index].iter().all(|h| h.is_none()) {
            return false;
        }
    }

    let room_state = state.get_room_state(target);
    room_state.iter().all(|s| s.is_none() || Some(target).eq(s))
}

fn find_least_energy_sort(initial_state: &State) -> Option<State> {
    // Build a shortest path with stack.
    // - Order by the used energy
    // - From each state, derive possible new states by:
    //   - Moving from a wrong room into reachable free spots in the hallway
    //   - Moving from the hallway to a reachable correct room (= empty or already filled with a right amphipod)
    // - Finish when finding a finished state, should be the cheapest by the algorithm
    // - Discard state when no moves possible

    fn create_move_out_states(state: &State) -> Vec<State> {
        fn create_state(state: &State, from: Amphipod) -> Vec<State> {
            let mut new_states = vec![];

            for idx in get_hallway_options(from, &state.hallway) {
                let mut new_state = state.clone();
                let from_room = new_state.get_room_state_mut(from);
                
                for ridx in 0..state.room_size {
                    if let Some(p) = from_room[ridx] {
                        from_room[ridx] = None;
                        new_state.hallway[idx] = Some(p);
                        new_state.used_energy += p.get_cost_between(from, ridx, idx);
                        break;
                    }
                }
                
                new_states.push(new_state);
            }

            new_states
        }

        fn get_states_from_room(state: &State, room: Amphipod) -> Vec<State> {
            let room_state = state.get_room_state(room);
            
            for r_idx in 0..state.room_size {
                if let Some(p) = room_state[r_idx] {
                    if p != room {
                        return create_state(state, room);
                    }
                }
            }
            
            vec![]
        }

        let a_states = get_states_from_room(state, Amphipod::A);
        let b_states = get_states_from_room(state, Amphipod::B);
        let c_states = get_states_from_room(state, Amphipod::C);
        let d_states = get_states_from_room(state, Amphipod::D);

        a_states.into_iter()
            .chain(b_states.into_iter())
            .chain(c_states.into_iter())
            .chain(d_states.into_iter())
            .collect()
    }
    
    fn create_move_in_states(state: &State) -> Vec<State> {
        // For each amphipod in the hallway, check if there is an option to move to it's destination room.
        let mut new_states = vec![];
        
        for idx in 0..7 {
            if let Some(p) = state.hallway[idx] {
                if can_move_into_room(state, idx) {
                    let mut new_state = state.clone();
                    new_state.hallway[idx] = None;
                    let room_state = new_state.get_room_state_mut(p);
                    for ridx in (0..state.room_size).rev() {
                        if room_state[ridx] == None {
                            room_state[ridx] = Some(p);
                            new_state.used_energy += p.get_cost_between(p, ridx, idx);
                            break;
                        }
                    }
                    new_states.push(new_state);
                }
            }
        }
        
        new_states
    }

    fn create_next_states(state: &State) -> Vec<State> {
        // 1. Check the top-most rooms; if there is a wrong amphipod there or below it,
        //    generate states for it moving to every reachable hallway spot.
        // 2. Do the same for the free bottom rooms.
        let move_out_states = create_move_out_states(state);
        // 3. Check the hallway, see if there is any amphipod in this state that could move into its room
        let move_in_states = create_move_in_states(state);
        
        move_out_states.into_iter().chain(move_in_states.into_iter()).collect()
    }

    let mut queue = BinaryHeap::new();
    let mut seen_states = HashMap::<State, usize>::new();
    queue.push(initial_state.clone());

    while let Some(state) = queue.pop() {
        if state.is_finished() {
            return Some(state)
        }
        
        if let Some(used_energy) = seen_states.get(&state) {
            if state.used_energy.ge(used_energy) {
                continue; // We've been here, but quicker.
            }
        }
        
        seen_states.insert(state.clone(), state.used_energy.clone());

        create_next_states(&state).into_iter().for_each(|s| queue.push(s));
    }
    
    None
}

#[cfg(test)]
mod tests {
    use crate::days::day23::Amphipod::{A, B, C, D};
    use crate::days::day23::{Amphipod, find_least_energy_sort, get_hallway_options, State};

    const EXAMPLE_INPUT: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_parse_format() {
        let state: State = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(state, State {
            hallway: [None; 7],
            room_a: [Some(B), Some(A), None, None],
            room_b: [Some(C), Some(D), None, None],
            room_c: [Some(B), Some(C), None, None],
            room_d: [Some(D), Some(A), None, None],
            room_size: 2,
            used_energy: 0,
        });

        assert_eq!(format!("{}", state), EXAMPLE_INPUT);
        assert_eq!(format!("{:?}", state), EXAMPLE_INPUT);
    }

    #[test]
    fn test_get_hallway_options() {
        assert_eq!(get_hallway_options(Amphipod::A, &[None; 7]), vec![1, 0, 2, 3, 4, 5, 6]);

        let hallway = [None, Some(B), None, None, Some(C), Some(A), None];
        assert_eq!(get_hallway_options(Amphipod::A, &hallway), vec![2, 3]);
        assert_eq!(get_hallway_options(Amphipod::B, &hallway), vec![2, 3]);
        assert_eq!(get_hallway_options(Amphipod::C, &hallway), vec![3, 2]);
        assert_eq!(get_hallway_options(Amphipod::D, &hallway), Vec::<usize>::new());
    }
    
    #[test]
    fn test_get_cost_between() {
        assert_eq!(Amphipod::B.get_cost_between(C, 0, 2), 40);
        assert_eq!(Amphipod::C.get_cost_between(B, 0, 3), 200);
        assert_eq!(Amphipod::C.get_cost_between(C, 0, 3), 200);
        assert_eq!(Amphipod::D.get_cost_between(B, 1, 3), 3000);
        assert_eq!(Amphipod::B.get_cost_between(C, 1, 2), 50);
        assert_eq!(Amphipod::B.get_cost_between(C, 2, 2), 60);
        assert_eq!(Amphipod::B.get_cost_between(C, 3, 2), 70);
    }
    
    #[test]
    fn test_is_finished() {
        let mut state = State {
            room_a: [Some(B), Some(A), None, None],
            room_b: [Some(C), Some(B), None, None],
            room_c: [Some(A), Some(C), None, None],
            room_d: [Some(D), Some(D), None, None],
            room_size: 2,
            ..State::default()
        };
        
        assert_eq!(state.is_finished(), false);
        state.room_a[0] = Some(A);
        state.room_c[0] = Some(B);
        assert_eq!(state.is_finished(), false);
        state.room_c[0] = Some(C);
        state.room_b[0] = Some(B);
        assert_eq!(state.is_finished(), true);
    }
    
    #[test]
    fn test_find_least_energy_sort() {
        let state: State = EXAMPLE_INPUT.parse().unwrap();
        
        let result = find_least_energy_sort(&state);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().used_energy, 12521);
    }
    
    #[test]
    fn test_find_least_energy_sort_full() {
        let mut state: State = EXAMPLE_INPUT.parse().unwrap();
        
        state.room_size = 4;
        state.room_a[3] = state.room_a[1];
        state.room_a[1] = Some(D);
        state.room_a[2] = Some(D);
        state.room_b[3] = state.room_b[1];
        state.room_b[1] = Some(C);
        state.room_b[2] = Some(B);
        state.room_c[3] = state.room_c[1];
        state.room_c[1] = Some(B);
        state.room_c[2] = Some(A);
        state.room_d[3] = state.room_d[1];
        state.room_d[1] = Some(A);
        state.room_d[2] = Some(C);
        
        let result = find_least_energy_sort(&state);
        assert!(result.is_some());
        assert_eq!(result.unwrap().used_energy, 44169);
    }
}