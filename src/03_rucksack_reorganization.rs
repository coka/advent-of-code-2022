use std::collections::HashSet;
use std::convert::TryFrom;

trait EncodeToSet {
    fn encode(&self) -> HashSet<char>;
}

impl EncodeToSet for str {
    fn encode(&self) -> HashSet<char> {
        self.chars().collect()
    }
}

fn intersect(s1: &HashSet<char>, s2: &HashSet<char>) -> HashSet<char> {
    s1.intersection(&s2).cloned().collect()
}

fn find_common_element(sets: Vec<HashSet<char>>) -> char {
    let (head, tail) = sets.split_at(1);
    let mut intersection = head[0].clone();
    for set in tail {
        intersection = intersect(&intersection, set);
    }
    *intersection.iter().next().unwrap()
}

trait Prioritize {
    fn priority(&self) -> u32;
}

const ITEM_TYPES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl Prioritize for char {
    fn priority(&self) -> u32 {
        u32::try_from(ITEM_TYPES.chars().position(|c| c == *self).unwrap()).unwrap() + 1
    }
}

struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl Rucksack {
    fn new(contents: &str) -> Self {
        let (left, right) = contents.split_at(contents.len() / 2);
        Rucksack {
            left: left.encode(),
            right: right.encode(),
        }
    }
}

pub fn prioritize_rearrangement(input: String) -> u32 {
    input
        .lines()
        .map(Rucksack::new)
        .map(|r| find_common_element([r.left, r.right].to_vec()).priority())
        .sum()
}

pub fn prioritize_badges(input: String) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            find_common_element(group.iter().map(|contents| contents.encode()).collect()).priority()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/03_example.txt").unwrap();
        assert_eq!(prioritize_rearrangement(input), 157);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input/03.txt").unwrap();
        assert_eq!(prioritize_rearrangement(input), 7831);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/03_example.txt").unwrap();
        assert_eq!(prioritize_badges(input), 70);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input/03.txt").unwrap();
        assert_eq!(prioritize_badges(input), 2683);
    }
}
