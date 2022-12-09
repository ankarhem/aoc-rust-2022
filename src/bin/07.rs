use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(u64, String),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

#[derive(Debug, PartialEq, Eq)]
struct Day7ParseError;

impl FromStr for Command {
    type Err = Day7ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next() {
            Some("ls") => Ok(Command::Ls),
            Some("cd") => Ok(Command::Cd(words.next().unwrap().to_string())),
            _ => Err(Day7ParseError),
        }
    }
}

impl FromStr for Entry {
    type Err = Day7ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        if let (Some(first), Some(second)) = (words.next(), words.next()) {
            match first {
                "dir" => Ok(Entry::Dir(second.to_string())),
                _ => {
                    let size = first.parse::<u64>().map_err(|_| Day7ParseError)?;
                    Ok(Entry::File(size, second.to_string()))
                }
            }
        } else {
            Err(Day7ParseError)
        }
    }
}

impl FromStr for Line {
    type Err = Day7ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first_char = chars.next().ok_or(Day7ParseError)?;
        match first_char {
            '$' => {
                let command = chars.collect::<String>().trim().parse::<Command>()?;
                Ok(Line::Command(command))
            }
            _ => {
                let entry = s.parse::<Entry>()?;
                Ok(Line::Entry(entry))
            }
        }
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, usize>, Day7ParseError> {
    let mut folders: HashMap<String, usize> = HashMap::new();
    let mut current_folder = vec![String::from("/")];
    for line in input.lines() {
        let parsed = line.parse::<Line>()?;

        match parsed {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(dir) => match dir.as_str() {
                    // Do nothing on cd /
                    "/" => {}
                    ".." => {
                        if current_folder.len() > 1 {
                            current_folder.pop();
                        }
                    }
                    _ => {
                        current_folder.push(dir);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                // Do nothing on dirs
                Entry::Dir(dir) => {
                    let path = current_folder.join("/") + &dir;
                    folders.entry(path).or_insert(0);
                }
                Entry::File(size, _name) => {
                    for i in 0..current_folder.len() {
                        let path = current_folder[..i + 1].join("/");
                        folders
                            .entry(path)
                            .and_modify(|value| *value += size as usize)
                            .or_insert(size as usize);
                    }
                }
            },
        }
    }

    Ok(folders)
}

pub fn part_one<'a>(input: &str) -> Option<u32> {
    let folders = parse_input(input).ok()?;
    let over_100k: usize = folders.values().filter(|&&size| size <= 100_000).sum();

    Some(over_100k as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    const TOTAL_DISK_SPACE: usize = 70_000_000;
    const SPACE_NEEDED: usize = 30_000_000;
    let folders = parse_input(input).ok()?;

    let &current_disk_size = folders.get("/")?;
    let need_to_free = SPACE_NEEDED - (TOTAL_DISK_SPACE - current_disk_size);

    // dir to delete
    let (_, size) = folders
        .iter()
        .filter(|(_, &size)| size >= need_to_free)
        .min_by_key(|(_, &size)| size)?;

    Some(*size as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
