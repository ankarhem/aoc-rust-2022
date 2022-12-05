use regex::Regex;
use std::str::FromStr;
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let digits = regex
            .replace(s, "$1 $2 $3")
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let (amount, from, to) = (digits[0], digits[1] - 1, digits[2] - 1);
        Ok(Move { amount, from, to })
    }
}

fn parse_input_parts(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut input_parts = input.split("\n\n");
    let (stacks_input, moves_input) = (input_parts.next().unwrap(), input_parts.next().unwrap());

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let lines = stacks_input.lines().rev().skip(1);
    lines.for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if !c.is_alphabetic() {
                return;
            }
            let stack_index = (i - 1) / 4;
            // push into the stack
            stacks[stack_index].push(c);
        })
    });

    let moves = moves_input
        .lines()
        .map(|s| s.parse::<Move>().unwrap())
        .collect::<Vec<Move>>();

    (stacks, moves)
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|s| {
            let char = s.clone().pop();

            match char {
                Some(c) => c.to_string(),
                None => "".to_string(),
            }
        })
        .collect::<Vec<String>>()
        .concat()
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input_parts(input);

    moves.iter().for_each(|m| {
        let mut from_stack = stacks[m.from as usize].clone();
        let mut to_stack = stacks[m.to as usize].clone();
        let mut amount = m.amount;
        while amount > 0 {
            let top = from_stack.pop().unwrap();
            to_stack.push(top);
            amount -= 1;
        }
        stacks[m.from as usize] = from_stack;
        stacks[m.to as usize] = to_stack;
    });

    let top_crates = get_top_crates(&stacks);
    Some(top_crates)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input_parts(input);

    moves.iter().for_each(|m| {
        let mut from_stack = stacks[m.from as usize].clone();
        let mut to_stack = stacks[m.to as usize].clone();
        let popped = from_stack
            .drain((from_stack.len() - m.amount as usize)..)
            .collect::<Vec<char>>();
        to_stack.extend(popped);
        stacks[m.from as usize] = from_stack;
        stacks[m.to as usize] = to_stack;
    });

    let top_crates = get_top_crates(&stacks);
    Some(top_crates)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
