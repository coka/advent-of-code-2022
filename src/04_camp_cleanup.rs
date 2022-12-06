use std::convert::TryInto;
use std::ops::RangeInclusive;

type Section = RangeInclusive<u32>;

fn parse_line(line: &str) -> (Section, Section) {
    let mut sections = line.split(',');
    let mut s1 = sections.next().unwrap().split('-');
    let mut s2 = sections.next().unwrap().split('-');
    (
        s1.next().unwrap().parse::<u32>().unwrap()..=s1.next().unwrap().parse::<u32>().unwrap(),
        s2.next().unwrap().parse::<u32>().unwrap()..=s2.next().unwrap().parse::<u32>().unwrap(),
    )
}

fn fully_contains(s1: &Section, s2: &Section) -> bool {
    s1.contains(s2.start()) && s1.contains(s2.end())
}

pub fn count_full_contains(input: String) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter(|(s1, s2)| fully_contains(s1, s2) || fully_contains(s2, s1))
        .count()
        .try_into()
        .unwrap()
}

fn overlaps(s1: &Section, s2: &Section) -> bool {
    s1.contains(s2.start()) || s1.contains(s2.end())
}

pub fn count_overlaps(input: String) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter(|(s1, s2)| overlaps(s1, s2) || overlaps(s2, s1))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/04_example.txt").unwrap();
        assert_eq!(count_full_contains(input), 2);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input/04.txt").unwrap();
        assert_eq!(count_full_contains(input), 477);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/04_example.txt").unwrap();
        assert_eq!(count_overlaps(input), 4);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input/04.txt").unwrap();
        assert_eq!(count_overlaps(input), 830);
    }
}
