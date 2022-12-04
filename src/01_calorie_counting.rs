fn parse(input: String) -> Vec<u32> {
    let mut result = Vec::new();
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(count);
            count = 0;
        } else {
            count += line.parse::<u32>().unwrap();
        }
    }
    result.push(count);
    result
}

pub fn count_calories(input: String, elves: usize) -> u32 {
    let mut calories = parse(input);
    // Because addition is commutative, we don't need to fully sort the elvish
    // inventory. Instead, we rely on the fact that all calorie counts we care
    // about are in the correct partition.
    //
    // https://en.wikipedia.org/wiki/Quickselect
    let pivot = calories.len() - elves;
    let (_, pivotal_elf, questionable_elves) = calories.select_nth_unstable(pivot);
    // Get it? They're questionable because we're _asking_ them for snacks!
    *pivotal_elf + questionable_elves.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one_example() {
        let input = fs::read_to_string("input/01_example.txt").unwrap();
        assert_eq!(count_calories(input, 1), 24000);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input/01.txt").unwrap();
        assert_eq!(count_calories(input, 1), 71780);
    }

    #[test]
    fn part_two_example() {
        let input = fs::read_to_string("input/01_example.txt").unwrap();
        assert_eq!(count_calories(input, 3), 45000);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input/01.txt").unwrap();
        assert_eq!(count_calories(input, 3), 212489);
    }
}
