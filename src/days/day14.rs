use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;

pub const DAY14: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    let result = compute_score(&puzzle, 10);

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    let result = compute_score(&puzzle, 40);

    println!("Puzzle 2 answer: {}", result);
}

struct PairInsertion {
    pair: [char; 2],
    insertion: char,
}

impl FromStr for PairInsertion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        if parts.len() != 2 || parts[0].len() != 2 || parts[1].len() != 1 {
            return Err(format!("Invalid format: {}", s));
        }

        Ok(PairInsertion { pair: parts[0].chars().collect::<Vec<char>>().try_into().unwrap(), insertion: parts[1].chars().next().unwrap() })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Puzzle {
    template: String,
    pair_insertions: HashMap<[char; 2], char>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct CacheKey {
    levels: usize,
    pair: [char; 2],
}

#[derive(Debug)]
struct CachePair {
    pair: [char; 2],
    occurrences: usize,
}

struct CacheEntry {
    /// The pairs this entry has at the end of the leafs
    pairs: Vec<CachePair>
}

fn compute_score(puzzle: &Puzzle, num_steps: usize) -> usize {
    // The length of the template grows too fast to use the simple way used above... (Of course. Why would it be simple?)
    // Idea:
    // For each pair, build a tree by computing the left and right pairs, until `num_steps` depth, counting characters.
    // That idea was still too slow. We're counting billions of characters due to the exponential nature of this...
    // Idea 2:
    // Memoization? Can we memoize somehow? We kinda can compute 5 or 10-depth relatively easily; could we utilize that?
    // We could compute 5-level trees for all pairs in puzzle, and use that memoization to get the right answers?
    // This is kinda hard-coding num_steps to be multiples of 5 or 10, which fits the puzzles.

    fn handle_pair(pair: [char; 2], num_steps: usize, pairs: &HashMap<[char; 2], char>) -> Vec<[char; 2]> {
        let insert = pairs.get(&pair);

        if num_steps == 0 || insert.is_none() {
            return vec![pair];
        }

        // Otherwise, prepare the next level of left/right pairs and dive in.
        let insert_char = insert.unwrap().clone();

        vec![[pair[0], insert_char], [insert_char, pair[1]]].into_iter().flat_map(|p| handle_pair(p, num_steps - 1, pairs)).collect()
    }

    let mut cache: HashMap<CacheKey, CacheEntry> = HashMap::new();

    // Level goes exponential:
    // 0    1    2    3     4      5
    // 1 => 2 => 4 => 8 => 16 => 32
    for level in 0..6 {
        let previous_level = if level == 0 { 0 } else { 1 << (level - 1) };
        let cache_level = 1 << level;
        for pair in puzzle.pair_insertions.iter() {
            if level == 0 {
                // Initial level caching
                // let mut values = HashMap::new();
                let pairs = handle_pair(pair.0.clone(), 1, &puzzle.pair_insertions);
                let unique_pairs = pairs.deduplicate();
                let cache_pairs: Vec<CachePair> = unique_pairs.into_iter().map(|p| CachePair { pair: p, occurrences: pairs.iter().cloned().filter(|o| o.eq(&p)).count() }).collect();
                cache.insert(CacheKey { levels: cache_level, pair: pair.0.clone() }, CacheEntry { pairs: cache_pairs });
            } else {
                // We should be able to get the last level from cache, and build a new entry based on it.
                let entry = cache.get(&CacheKey { levels: previous_level, pair: pair.0.clone() }).unwrap();

                // let mut values = entry.values.clone();
                let mut pairs: HashMap<[char; 2], usize> = HashMap::new();

                for pair in &entry.pairs {
                    let entry = cache.get(&CacheKey { levels: previous_level, pair: pair.pair }).unwrap();
                    for entry_pair in &entry.pairs {
                        pairs.insert(entry_pair.pair, pairs.get(&entry_pair.pair).unwrap_or(&0) + entry_pair.occurrences * pair.occurrences);
                    }
                }

                let cache_pairs: Vec<CachePair> = pairs.iter().map(|p| CachePair { pair: p.0.clone(), occurrences: p.1.clone() }).collect();
                cache.insert(CacheKey { levels: cache_level, pair: pair.0.clone() }, CacheEntry { pairs: cache_pairs });
            }
        }
    }

    fn compute(pair: &[char; 2], occurrences: usize, cache: &HashMap<CacheKey, CacheEntry>, num_steps: usize) -> Vec<CachePair> {
        if num_steps == 0 { return vec![{ CachePair { pair: pair.clone(), occurrences }}]; }

        let cache_steps = if num_steps >= 32 { 32 } else if num_steps >= 16 { 16 } else if num_steps >= 8 { 8 } else if num_steps >= 4 { 4 } else if num_steps >= 2 { 2 } else { 1 };

        let entry = cache.get(&CacheKey { levels: cache_steps, pair: pair.clone() }).unwrap();

        let mut pairs: HashMap<[char; 2], usize> = HashMap::new();

        for entry_pair in &entry.pairs {
            for cache_pair in compute(&entry_pair.pair, entry_pair.occurrences * occurrences, cache, num_steps - cache_steps) {
                pairs.insert(cache_pair.pair, pairs.get(&cache_pair.pair).unwrap_or(&0) + cache_pair.occurrences);
            }
        }

        pairs.iter().map(|p| CachePair { pair: p.0.clone(), occurrences: p.1.clone() }).collect()
    }

    let mut buckets: HashMap<[char; 2], usize> = HashMap::new();

    let chars: Vec<char> = puzzle.template.chars().collect();
    for i in 0..chars.len() - 1 {
        for pair in compute(&[chars[i], chars[i+1]], 1, &cache, num_steps) {
            buckets.insert(pair.pair, buckets.get(&pair.pair).unwrap_or(&0) + pair.occurrences);
        }
    }

    // Character counts, we need to count the first character of the initial template ourselves, and then all second characters of the pairs.
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    char_counts.insert(chars[0], 1);
    for bucket_entry in buckets.iter() {
        let bucket_char = bucket_entry.0[1];
        char_counts.insert(bucket_char, char_counts.get(&bucket_char).unwrap_or(&0) + bucket_entry.1);
    }

    let scores: Vec<(char, usize)> = char_counts.iter().map(|e| (e.0.clone(), e.1.clone())).collect();
    let max: usize = scores.iter().max_by_key(|p| p.1).map(|p| p.1).unwrap_or(0);
    let min: usize = scores.iter().min_by_key(|p| p.1).map(|p| p.1).unwrap_or(0);

    max - min
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid format: {}", s));
        }

        let template = parts[0].trim().to_owned();
        let mut pair_insertions = HashMap::new();

        parts[1].lines().filter(|l| !l.trim().is_empty())
            .filter_map(|l| l.parse::<PairInsertion>().ok())
            .for_each(|pi| { pair_insertions.insert(pi.pair, pi.insertion); });

        Ok(Puzzle { template, pair_insertions })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day14::{Puzzle, compute_score};

    const EXAMPLE_INPUT: &str = "\
        NNCB\n\
        \n\
        CH -> B\n\
        HH -> N\n\
        CB -> H\n\
        NH -> C\n\
        HB -> C\n\
        HC -> B\n\
        HN -> C\n\
        NN -> C\n\
        BH -> H\n\
        NC -> B\n\
        NB -> B\n\
        BN -> B\n\
        BB -> N\n\
        BC -> B\n\
        CC -> N\n\
        CN -> C\
    ";

    #[test]
    fn test_parse() {
        let result: Result<Puzzle, String> = EXAMPLE_INPUT.parse();

        assert_eq!(result, Ok(Puzzle {
            template: String::from("NNCB"),
            pair_insertions: HashMap::from([
                (['C', 'H'], 'B'),
                (['H', 'H'], 'N'),
                (['C', 'B'], 'H'),
                (['N', 'H'], 'C'),
                (['H', 'B'], 'C'),
                (['H', 'C'], 'B'),
                (['H', 'N'], 'C'),
                (['N', 'N'], 'C'),
                (['B', 'H'], 'H'),
                (['N', 'C'], 'B'),
                (['N', 'B'], 'B'),
                (['B', 'N'], 'B'),
                (['B', 'B'], 'N'),
                (['B', 'C'], 'B'),
                (['C', 'C'], 'N'),
                (['C', 'N'], 'C')
            ]),
        }))
    }

    #[test]
    fn test_score() {
        let puzzle: Puzzle = EXAMPLE_INPUT.parse().unwrap();

        assert_eq!(compute_score(&puzzle, 10), 1588);
        assert_eq!(compute_score(&puzzle, 40), 2188189693529);
    }
}