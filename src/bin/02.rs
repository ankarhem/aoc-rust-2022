enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum MatchResult {
    Loss,
    Draw,
    Win,
}

fn calc_score(player_one: Shape, player_two: Shape) -> u32 {
    let match_result = match player_one {
        Shape::Rock => match player_two {
            Shape::Rock => MatchResult::Draw,
            Shape::Paper => MatchResult::Win,
            Shape::Scissors => MatchResult::Loss,
        },
        Shape::Paper => match player_two {
            Shape::Rock => MatchResult::Loss,
            Shape::Paper => MatchResult::Draw,
            Shape::Scissors => MatchResult::Win,
        },
        Shape::Scissors => match player_two {
            Shape::Rock => MatchResult::Win,
            Shape::Paper => MatchResult::Loss,
            Shape::Scissors => MatchResult::Draw,
        },
    };

    let shape_score = match player_two {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let match_score = match match_result {
        MatchResult::Loss => 0,
        MatchResult::Draw => 3,
        MatchResult::Win => 6,
    };

    match_score + shape_score
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| {
            let mut round = line.split_whitespace();

            let player_one = round.next().unwrap();
            let player_two = round.next().unwrap();

            let player_one = match player_one {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!("Invalid input: {}", player_one),
            };

            let player_two = match player_two {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!("Invalid input {}", player_two),
            };

            calc_score(player_one, player_two)
        })
        .sum::<u32>();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| {
            let mut round = line.split_whitespace();

            let player_one = round.next().unwrap();
            let player_two = round.next().unwrap();

            let player_one = match player_one {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!("Invalid input: {}", player_one),
            };

            let player_two = match player_two {
                "X" => match player_one {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                },
                "Y" => match player_one {
                    Shape::Rock => Shape::Rock,
                    Shape::Paper => Shape::Paper,
                    Shape::Scissors => Shape::Scissors,
                },
                "Z" => match player_one {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                },
                _ => panic!("Invalid input {}", player_two),
            };
            calc_score(player_one, player_two)
        })
        .sum::<u32>();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
