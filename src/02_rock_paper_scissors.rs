// Move values are used as indices.
#[derive(Clone, Copy)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

use self::Move::*;

impl Move {
    fn parse(c: char) -> Move {
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

use self::Outcome::*;

impl Outcome {
    fn parse(c: char) -> Outcome {
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

// Rows correspond to player moves, columns to opponent moves.
const OUTCOMES: [[Outcome; 3]; 3] = [
    // (force formatting)
    [Draw, Loss, Win],
    [Win, Draw, Loss],
    [Loss, Win, Draw],
];

fn parse_rounds(guide: String) -> Vec<(Move, Move)> {
    let mut rounds = Vec::new();
    for line in guide.lines() {
        let mut chars = line.chars();
        let opponent = Move::parse(chars.nth(0).unwrap());
        let player = Move::parse(chars.nth(1).unwrap());
        rounds.push((player, opponent));
    }
    rounds
}

pub fn follow_strategy(guide: String) -> u32 {
    parse_rounds(guide)
        .iter()
        .map(|&round| {
            let (player, opponent) = round;
            let outcome = OUTCOMES[player as usize][opponent as usize];
            outcome.score() + player.score()
        })
        .sum()
}

// Rows correspond to opponent moves, columns to desired outcomes.
const MOVES: [[Move; 3]; 3] = [
    [Paper, Scissors, Rock],
    [Scissors, Rock, Paper],
    [Rock, Paper, Scissors],
];

fn correctly_parse_rounds(guide: String) -> Vec<(Move, Outcome)> {
    let mut rounds = Vec::new();
    for line in guide.lines() {
        let mut chars = line.chars();
        let opponent = Move::parse(chars.nth(0).unwrap());
        let outcome = Outcome::parse(chars.nth(1).unwrap());
        rounds.push((opponent, outcome));
    }
    rounds
}

pub fn follow_correct_strategy(guide: String) -> u32 {
    correctly_parse_rounds(guide)
        .iter()
        .map(|&round| {
            let (opponent, outcome) = round;
            let player = MOVES[opponent as usize][outcome as usize];
            outcome.score() + player.score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/02_example.txt").unwrap();
        assert_eq!(follow_strategy(input), 15);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input/02.txt").unwrap();
        assert_eq!(follow_strategy(input), 10718);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/02_example.txt").unwrap();
        assert_eq!(follow_correct_strategy(input), 12);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input/02.txt").unwrap();
        assert_eq!(follow_correct_strategy(input), 14652);
    }
}
