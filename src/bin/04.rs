use std::str::FromStr;

struct Section {
    start: u32,
    end: u32,
}
struct ElfPair {
    first: Section,
    second: Section,
}

impl FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Ok(Section { start, end })
    }
}

impl FromStr for ElfPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split(",").map(|s| s.parse::<Section>().unwrap());

        let first = sections.next().unwrap();
        let second = sections.next().unwrap();

        Ok(ElfPair {
            first,
            second,
        })
    }
}

fn parse_sections(input: &str) -> Vec<ElfPair> {
    let sections = input
        .lines()
        .map(|s| s.parse::<ElfPair>().unwrap())
        .collect::<Vec<ElfPair>>();
    sections
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_pairs = parse_sections(input);

    let full_overlaps = elf_pairs.iter().filter(|pair| {
        (pair.first.start <= pair.second.start && pair.first.end >= pair.second.end)
            || (pair.second.start <= pair.first.start && pair.first.end <= pair.second.end)
    });

    Some(full_overlaps.count().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_pairs = parse_sections(input);

    let overlaps = elf_pairs.iter().filter(|pair| {
        (pair.first.start >= pair.second.start && pair.first.start <= pair.second.end)
            || (pair.second.start >= pair.first.start && pair.second.start <= pair.first.end)
    });

    Some(overlaps.count().try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
