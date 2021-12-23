use std::collections::HashMap;
use std::ops::Mul;
use std::str::FromStr;
use num_bigint::BigUint;
use num_traits::{One,Zero};
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY21: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut game: Game = input.parse().unwrap();

    play_deterministic(&mut game);

    let losing_score = if game.player_one_score >= 1000 { game.player_two_score } else { game.player_one_score };
    let result = losing_score * game.dice_rolls;

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let game: Game = input.parse().unwrap();

    let result = play_dirac(&game, 21);

    println!("Puzzle 2 answer: {}", result.num_universes_player_one.max(result.num_universes_player_two));
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Game {
    player_one_position: usize,
    player_one_score: usize,
    player_two_position: usize,
    player_two_score: usize,
    dice_rolls: usize,
    turn: usize,
}

impl Game {
    fn new(player_one_position: usize, player_two_position: usize) -> Self {
        Self {
            player_one_position,
            player_one_score: 0,
            player_two_position,
            player_two_score: 0,
            dice_rolls: 0,
            turn: 1
        }
    }

    fn finished(&self, target_score: usize) -> bool {
        self.player_one_score >= target_score || self.player_two_score >= target_score
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: [&str; 2] = s.lines().collect::<Vec<&str>>().try_into().map_err(|v: Vec<&str>| format!("Incorrect number of lines: {}", v.len()))?;

        if !lines[0].starts_with("Player 1 starting position: ") {
            return Err(format!("Invalid first line: {}", lines[0]));
        }
        if !lines[1].starts_with("Player 2 starting position: ") {
            return Err(format!("Invalid first line: {}", lines[0]));
        }

        let player_one_position = parse_usize(&lines[0][28..])?;
        let player_two_position = parse_usize(&lines[1][28..])?;
        Ok(Game::new(player_one_position, player_two_position))
    }
}

fn play_deterministic(game: &mut Game) {
    // Play with the deterministic die; a 100-sided die that will roll 1,2,3,...,100,1,...,etc
    // The game is a board with 10 spots, 1-10
    // The game starts with player 1.
    // Each turn:
    // - Current player rolls the die three times, and adds the values
    // - The player moved from it's current spot by the total amount rolled
    // - The spot the player ends on is their score for this turn.
    // The game is over when either player hits a score >= 1000

    // There might be a smart way to run this; but even at a worst-case of 1000 turns this is not
    // too interesting too optimize.

    // We're assuming a fresh game here.
    let mut turn = 1;

    while !game.finished(1000) {
        let first_number = (game.dice_rolls % 100) + 1;
        let second_number = (first_number % 100) + 1;
        let third_number = (second_number % 100) + 1;
        game.dice_rolls += 3;

        let total_value = first_number + second_number + third_number;

        let (position, score) = match turn {
            1 => (&mut game.player_one_position, &mut game.player_one_score),
            2 => (&mut game.player_two_position, &mut game.player_two_score),
            _ => panic!("Oopsie daisy!")
        };

        *position = ((*position - 1 + total_value) % 10) + 1;
        *score += *position;
        turn = (turn % 2) + 1;
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct GameResult {
    num_universes_player_one: BigUint,
    num_universes_player_two: BigUint,
}

fn play_dirac(game: &Game, target_score: usize) -> GameResult {
    // Oh boy. We now play a game with a three-sided die that will result in split universes.
    // We need to play till a score of 21, and find out in how many universes each player wins.
    // Given the example answers of 444356092776315 and 341960390180808 universes. I don't think
    // we can dumb-implement this :joy:

    // What do we know?
    // - We need to get to 21.
    // - We always advance 3..=9 steps (3,4,5,6,7,8,9 = 7 options)
    // - Every round, we have 7 new results
    //   - in 27 (3*3*3) universes (the order of thrown dices matters for the universes)
    //     this might be important, as it could severely reduce what we need to check
    //     1,2,3, 1,3,2 and 3,2,1 - for example - have the same outcome in a different universe
    // - ?

    // Universe spawning
    // Throws Result # of combinations
    // 1,1,1    3       1
    // 1,1,2    4       3 (1,1,2;1,2,1;2,1,1)
    // 1,2,2    5       3 (1,2,2;2,1,2;2,2,1)
    // 1,1,3    5       3
    // 2,2,2    6       1
    // 1,2,3    6       6 (3*2*1)
    // 1,3,3    7       3
    // 2,2,3    7       3
    // 2,3,3    8       3
    // 3,3,3    9       1
    // options: 7      27
    // 3 = 1, 4 = 3, 5 = 6, 6 = 7, 7 = 6, 8 = 3, 9 = 1 universe/score

    fn factor(val: usize) -> usize {
        // Different combinations of three three-sided dice throws yielding the given value...
        match val {
            3 | 9 => 1,
            4 | 8 => 3,
            5 | 7 => 6,
            6     => 7,
            _ => panic!("Invalid value {}", val)
        }
    }

    // How do we determine in a reasonable time how many universes the players win?
    // Can we do this for the 7 score options?
    // If we check every result for player 1, and then for two, and build the lists of scores that get to a win
    //  we can compute in how many universes that happens.
    // We still need some extra smarts here.
    // Can we cache/memo something?
    // There is most likely some repetition in game state.

    fn compute_games(game: &Game, cache: &mut HashMap<Game, GameResult>, target_score: usize) -> GameResult {
        if game.finished(target_score) {
            return if game.player_one_score >= target_score {
                GameResult { num_universes_player_one: BigUint::one(), num_universes_player_two: BigUint::zero() }
            } else {
                GameResult { num_universes_player_one: BigUint::zero(), num_universes_player_two: BigUint::one() }
            }
        }

        // If the current game is cached, return cached values.
        if let Some(result) = cache.get(game) {
            return result.clone();
        }

        // Continue playing:
        let mut results = GameResult::default();

        for next in 3..=9 {
            let mut next_game = game.clone();
            let (position, score) = match game.turn {
                1 => (&mut next_game.player_one_position, &mut next_game.player_one_score),
                2 => (&mut next_game.player_two_position, &mut next_game.player_two_score),
                _ => panic!("Oopsie daisy!")
            };
            *position = ((*position - 1 + next) % 10) + 1;
            *score += *position;
            next_game.turn = next_game.turn % 2 + 1;

            let result = compute_games(&next_game, cache, target_score);

            // We need to multiply these by the factor of universes next has
            results.num_universes_player_one += result.num_universes_player_one.mul(factor(next));
            results.num_universes_player_two += result.num_universes_player_two.mul(factor(next));
        }

        // What can we cache?
        // From `game`, we have now tried all combinations. Meaning we know the total results
        // from this point on, which we can actually cache.
        cache.insert(game.clone(), results.clone());

        results
    }

    compute_games(game, &mut HashMap::new(), target_score)
}

#[cfg(test)]
mod tests {
    use crate::days::day21::{Game, GameResult, play_deterministic, play_dirac};

    #[test]
    fn test_parse() {
        let game = "\
            Player 1 starting position: 4\n\
            Player 2 starting position: 8\
        ".parse();

        assert_eq!(game, Ok(Game::new(4, 8)));
    }

    #[test]
    fn test_example_game() {
        let mut game = Game::new(4, 8);

        play_deterministic(&mut game);

        assert_eq!(game.player_one_score, 1000);
        assert_eq!(game.player_two_score, 745);
        assert_eq!(game.dice_rolls, 993);
    }

    #[test]
    fn test_dirac_game() {
        let game = Game::new(4, 8);

        // For now, we're interested in the speed of this.
        let GameResult { num_universes_player_one, num_universes_player_two } = play_dirac(&game, 21);

        println!("Game {:?} ended with {} vs {} wins", game, num_universes_player_one, num_universes_player_two);

        assert_eq!(num_universes_player_one, "444356092776315".parse().unwrap());
        assert_eq!(num_universes_player_two, "341960390180808".parse().unwrap());
    }
}