pub fn part_one(input: &str) -> Option<u32> {
    let priorities = input.lines().map(|rucksack| {
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);

        let shared_char = first_half
            .chars()
            .find_map(|c| {
                if second_half.contains(c) {
                    Some(c)
                } else {
                    None
                }
            });

        match shared_char {
            Some(c) => {
                if c.is_lowercase() {
                    Some((c as u32) - ('a' as u32) + 1)
                } else {
                    Some((c as u32) - ('A' as u32) + 27)
                }
            }
            None => None,
        }
    });

    priorities.sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    for idx in 0..(lines.len() / 3) {
        let (first, second, third) = (
            lines.get(idx * 3),
            lines.get(idx * 3 + 1),
            lines.get(idx * 3 + 2),
        );
        if first.is_none() || second.is_none() || third.is_none() {
            return None;
        }

        let (first, second, third) = (first.unwrap(), second.unwrap(), third.unwrap());

        let shared_char = first
            .chars()
            .find_map(|c| {
                if second.contains(c) && third.contains(c) {
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap();

        if shared_char.is_lowercase() {
            sum += (shared_char as u32) - ('a' as u32) + 1;
        } else {
            sum += (shared_char as u32) - ('A' as u32) + 27;
        }
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
