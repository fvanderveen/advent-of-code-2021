use crate::days::Day;

pub const DAY8: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let screens = match parse_input(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let result = count_simple_output_digits(screens);

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let screens = match parse_input(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let result: Result<Vec<usize>, String> = screens.iter().map(|s| determine_mapping(s).and_then(|m| Ok(compute_screen_output(s, &m)))).collect();
    let total_value: usize = match result {
        Ok(v) => v.iter().sum(),
        Err(e) => panic!("{}", e)
    };
    println!("Puzzle 2 answer: {}", total_value);
}

fn count_simple_output_digits(screens: Vec<Screen>) -> u128 {
    // Puzzle 1 is simple, count all occurrences of '1', '4', '7', or '8' in the outputs.
    // Those numbers have a unique amount of segments (2, 4, 3, and 7 respectively).
    screens.iter().map(|s| s.output.iter().map(|d| match d.len() {
        2 | 3 | 4 | 7 => 1,
        _ => 0
    }).sum::<u128>()).sum()
}

fn compute_screen_output(screen: &Screen, mapping: &[String; 10]) -> usize {
    let mut result = 0;
    for value in &screen.output {
        result *= 10;
        for i in 0..10 {
            if is_same_digit(&mapping[i], value) {
                result += i;
            }
        }
    }

    result
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Screen {
    all_digits: [String; 10],
    output: [String; 4],
}

fn parse_input(input: &str) -> Result<Vec<Screen>, String> {
    input.lines().map(|l| parse_screen(l)).collect()
}

fn parse_screen(input: &str) -> Result<Screen, String> {
    let split_result: Result<[&str; 2], Vec<_>> = input.split(" | ").collect::<Vec<_>>().try_into();
    let [input_digits, input_output] = match split_result {
        Ok([digits, output]) => [digits, output],
        Err(parts) => return Err(format!("Expected exactly two parts, but got {}", parts.len()))
    };

    let parsed_digits = parse_digits(input_digits);
    let parsed_output = parse_digits(input_output);

    match (parsed_digits, parsed_output) {
        (Ok(all_digits), Ok(output)) => to_screen(all_digits, output),
        (Err(e1), Err(e2)) => Err(format!("{} | {}", e1, e2)),
        (Err(e), _) | (_, Err(e)) => Err(e)
    }
}

fn parse_digits(input: &str) -> Result<Vec<String>, String> {
    input.split(" ").map(|d| parse_digit(d)).collect()
}

fn parse_digit(input: &str) -> Result<String, String> {
    let valid_chars: Result<Vec<char>, String> = input.chars().map(|c| match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' => Ok(c),
        _ => Err(format!("Invalid character {}", c))
    }).collect();

    match valid_chars {
        Ok(c) => Ok(c.iter().collect()),
        Err(e) => Err(e)
    }
}

fn to_screen(all_digits: Vec<String>, output: Vec<String>) -> Result<Screen, String> {
    let all_digits_arr: Result<[String; 10], String> = all_digits.try_into().map_err(|e: Vec<String>| format!("all_digits has wrong size {}", e.len()));
    let output_arr: Result<[String; 4], String> = output.try_into().map_err(|e: Vec<String>| format!("output has wrong size {}", e.len()));

    match (all_digits_arr, output_arr) {
        (Ok(ada), Ok(oa)) => Ok(Screen { all_digits: ada, output: oa }),
        (Err(e1), Err(e2)) => Err(format!("{} | {}", e1, e2)),
        (Err(e), _) | (_, Err(e)) => Err(e)
    }
}

fn determine_mapping(screen: &Screen) -> Result<[String; 10], String> {
    Ok([
        find_digit_zero(&screen.all_digits).unwrap(),
        find_digit_one(&screen.all_digits).unwrap(),
        find_digit_two(&screen.all_digits).unwrap(),
        find_digit_three(&screen.all_digits).unwrap(),
        find_digit_four(&screen.all_digits).unwrap(),
        find_digit_five(&screen.all_digits).unwrap(),
        find_digit_six(&screen.all_digits).unwrap(),
        find_digit_seven(&screen.all_digits).unwrap(),
        find_digit_eight(&screen.all_digits).unwrap(),
        find_digit_nine(&screen.all_digits).unwrap(),
    ])
}

fn sort_digit_chars(input: &String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    chars.iter().collect()
}

fn find_digit<P>(inputs: &[String; 10], digit_name: &str, predicate: P) -> Result<String, String>
    where P: FnMut(&String) -> bool
    {
    let matches: Vec<String> = inputs.into_iter()
        .map(|s| sort_digit_chars(s))
        .filter(predicate)
        .collect();
    match matches.len() {
        1 => Ok(matches[0].clone()),
        len => Err(format!("Could not find a mapping for {}, found {} candidates", digit_name, len))
    }
}

fn get_overlap_chars(left: &String, right: &String) -> Vec<char> {
    let right_chars: Vec<char> = right.chars().collect();
    left.chars().filter(|c| right_chars.contains(c)).collect()
}

fn is_same_digit(left: &String, right: &String) -> bool {
    left.len() == right.len() && get_overlap_chars(left, right).len() == left.len()
}

fn find_digit_zero(inputs: &[String; 10]) -> Result<String, String> {
    // All non-middle segments
    let middle_segment = get_middle_segment(inputs).unwrap();
    find_digit(inputs, "0", |d| d.len() == 6 && !d.contains(middle_segment))
}

fn find_digit_one(inputs: &[String; 10]) -> Result<String, String> {
    // Only digit with two segments
    find_digit(inputs, "1", |d| d.len() == 2)
}

fn find_digit_two(inputs: &[String; 10]) -> Result<String, String> {
    // The 5-length option that has only two matching characters with 4
    let four = find_digit_four(inputs).unwrap();
    find_digit(inputs, "5", |d| d.len() == 5 && get_overlap_chars(d, &four).len() == 2)
}

fn find_digit_three(inputs: &[String; 10]) -> Result<String, String> {
    // We can find three by finding the only 5-length option that includes _both_ of digit one's characters.
    let one = find_digit_one(inputs).unwrap();
    find_digit(inputs, "1", |d| d.len() == 5 && one.chars().all(|c| d.contains(c)))
}

fn find_digit_four(inputs: &[String; 10]) -> Result<String, String> {
    // The only one with 4 segments.
    find_digit(inputs, "4", |d| d.len() == 4)
}

fn find_digit_five(inputs: &[String; 10]) -> Result<String, String> {
    // Since we can find three and two more easily, this is the 5-length string that is neither of those.
    let two = find_digit_two(inputs).unwrap();
    let three = find_digit_three(inputs).unwrap();

    find_digit(inputs, "5", |d| d.len() == 5 && d.ne(&two) && d.ne(&three))
}

fn find_digit_six(inputs: &[String; 10]) -> Result<String, String> {
    // All except the top-right segment.
    let top_right_segment = get_top_right_segment(inputs).unwrap();
    find_digit(inputs, "6", |d| d.len() == 6 && !d.contains(top_right_segment))
}

fn find_digit_seven(inputs: &[String; 10]) -> Result<String, String> {
    // The only one with 3 segments.
    find_digit(inputs, "7", |d| d.len() == 3)
}

fn find_digit_eight(inputs: &[String; 10]) -> Result<String, String> {
    // The only one with all segments.
    find_digit(inputs, "8", |d| d.len() == 7)
}

fn find_digit_nine(inputs: &[String; 10]) -> Result<String, String> {
    // The one 6-length that isn't 0 or 6 (too lazy to add a get-bottom-left)
    let zero = find_digit_zero(inputs).unwrap();
    let six = find_digit_six(inputs).unwrap();

    find_digit(inputs, "9", |d| d.len() == 6 && d.ne(&zero) && d.ne(&six))
}

fn get_top_segment(inputs: &[String; 10]) -> Result<char, String> {
    // Top segment is the one extra char that seven has over one.
    let seven = find_digit_seven(inputs).unwrap();
    let one = find_digit_one(inputs).unwrap();

    match seven.chars().find(|c| !one.contains(c.clone())) {
        Some(c) => Ok(c),
        None => Err("Could not determine the top-most bit".to_string())
    }
}

fn get_middle_segment(inputs: &[String; 10]) -> Result<char, String> {
    let top_segment = get_top_segment(inputs).unwrap();
    let bottom_segment = get_bottom_segment(inputs).unwrap();
    let one = find_digit_one(inputs).unwrap();
    let three = find_digit_three(inputs).unwrap();

    // Middle is the only char in three that is neither in 1, top, or bottom.
    let chars: Vec<char> = three.chars().filter(|c| c != &top_segment && c != &bottom_segment && !one.contains(c.clone())).collect();

    match chars.len() {
        1 => Ok(chars[0]),
        len => Err(format!("Could not get middle segment, {} chars matched: {}", len, chars.into_iter().map(String::from).collect::<String>()))
    }
}

fn get_bottom_segment(inputs: &[String; 10]) -> Result<char, String> {
    // Bottom segment is the only char common in all 5-length inputs that is not in 4 (and isn't top)
    // 2 -> top, two of 4, bottom-left, bottom
    // 3 -> top, three of 4, bottom
    // 5 -> top, three of 4, bottom
    let four_chars: Vec<char> = find_digit_four(inputs).unwrap().chars().collect();
    let top_segment = get_top_segment(inputs).unwrap();
    let relevant_inputs: Vec<&String> = inputs.iter().filter(|d| d.len() == 5).collect();
    relevant_inputs.iter()
        .map(|i| i.chars().filter(|c| c != &top_segment && !four_chars.contains(c)).collect::<Vec<char>>())
        .filter_map(|c| if c.len() == 1 { Some(c[0]) } else { None })
        .next()
        .ok_or(format!("Could not determine bottom segment"))
}

fn get_top_right_segment(inputs: &[String; 10]) -> Result<char, String> {
    // The only character from 1 that is also in 2.
    let one_chars: Vec<char> = find_digit_one(inputs).unwrap().chars().collect();
    let two_chars: Vec<char> = find_digit_two(inputs).unwrap().chars().collect();

    let matches: Vec<char> = one_chars.into_iter().filter(|c| two_chars.contains(c)).collect();
    match matches.len() {
        1 => Ok(matches[0]),
        len => Err(format!("Could not determine top-left segment, {} matches: {}", len, matches.into_iter().map(String::from).collect::<String>()))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::{compute_screen_output, count_simple_output_digits, determine_mapping, find_digit_eight, find_digit_five, find_digit_four, find_digit_nine, find_digit_one, find_digit_seven, find_digit_six, find_digit_three, find_digit_two, find_digit_zero, get_bottom_segment, get_top_right_segment, get_top_segment, parse_input, parse_screen, Screen};

    const EXAMPLE_INPUT: &str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\
    ";

    #[test]
    fn test_parse_screen() {
        assert_eq!(parse_screen("dcbgefa cebd bfega eadbf db cdfaeb dba bfcgda egadcf aedcf | egadcfb eafcd db debc"), Ok(Screen {
            all_digits: ["dcbgefa", "cebd", "bfega", "eadbf", "db", "cdfaeb", "dba", "bfcgda", "egadcf", "aedcf"].map(|s| s.to_string()),
            output: ["egadcfb", "eafcd", "db", "debc"].map(|s| s.to_string()),
        }));
    }

    #[test]
    fn test_puzzle_1() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(count_simple_output_digits(input), 26);
    }

    #[test]
    fn test_get_top_segment() {
        assert_eq!(get_top_segment(&["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"].map(str::to_string)), Ok('d'));
        assert_eq!(get_top_segment(&["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb", "cdgabef"].map(str::to_string)), Ok('b'));
    }

    #[test]
    fn test_get_bottom_segment() {
        assert_eq!(get_bottom_segment(&["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"].map(str::to_string)), Ok('f'));
        assert_eq!(get_bottom_segment(&["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb", "cdgabef"].map(str::to_string)), Ok('d'));
    }

    #[test]
    fn test_get_top_right_segment() {
        assert_eq!(get_top_right_segment(&["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"].map(str::to_string)), Ok('b'));
    }

    #[test]
    fn test_get_digits() {
        let inputs = ["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"].map(str::to_string);

        //  dddd
        // g    b
        // g    b
        //  cccc
        // a    e
        // a    e
        //  ffff

        assert_eq!(find_digit_zero(&inputs), Ok("abdefg".to_string()), "Wrong zero result");
        assert_eq!(find_digit_one(&inputs), Ok("be".to_string()), "Wrong one result");
        assert_eq!(find_digit_two(&inputs), Ok("abcdf".to_string()), "Wrong two result");
        assert_eq!(find_digit_three(&inputs), Ok("bcdef".to_string()), "Wrong three result");
        assert_eq!(find_digit_four(&inputs), Ok("bceg".to_string()), "Wrong four result");
        assert_eq!(find_digit_five(&inputs), Ok("cdefg".to_string()), "Wrong five result");
        assert_eq!(find_digit_six(&inputs), Ok("acdefg".to_string()), "Wrong six result");
        assert_eq!(find_digit_seven(&inputs), Ok("bde".to_string()), "Wrong seven result");
        assert_eq!(find_digit_eight(&inputs), Ok("abcdefg".to_string()), "Wrong eight result");
        assert_eq!(find_digit_nine(&inputs), Ok("bcdefg".to_string()), "Wrong nine result");
    }

    #[test]
    fn test_compute_screen_output() {
        let screens = parse_input(EXAMPLE_INPUT).unwrap();
        let mapping = determine_mapping(&screens[0]).unwrap();
        assert_eq!(compute_screen_output(&screens[0], &mapping), 8394);
    }
}