use std::num::ParseIntError;

use std::str::FromStr;
struct Elf {
    calories: u32,
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories = s
            .lines()
            .map(|line| line.parse::<u32>())
            .sum::<Result<u32, ParseIntError>>()?;
        Ok(Elf { calories })
    }
}

fn parse_to_elfs(input: &str) -> Vec<Elf> {
    let elfs = input
        .split("\n\n")
        .map(|elf| elf.to_owned().parse::<Elf>().unwrap())
        .collect();

    elfs
}

pub fn part_one(input: &str) -> Option<u32> {
    let elfs = parse_to_elfs(input);
    let max = elfs
        .iter()
        .max_by_key(|elf| elf.calories)
        .map(|elf| elf.calories);

    max
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elfs = parse_to_elfs(input);
    elfs.sort_by_key(|elf| elf.calories);
    let sum_top_3 = elfs.iter().map(|elf| elf.calories).rev().take(3).sum();

    Some(sum_top_3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
