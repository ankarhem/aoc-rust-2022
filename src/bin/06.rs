fn get_signal_marker(signal: &str, length: usize) -> Option<usize> {
    let mut index: usize = 0;
    'outer: while let Some(word) = signal.get(index..index + length) {
        for (i, c) in word.chars().enumerate() {
            if let Some(rest) = word.get(i + 1..) {
                if let Some(_idx_of_dup) = rest.find(c) {
                    // The rest of the string also contains the current
                    // character so we can increment the base index to the
                    // current index plus 1 and continue the outer loop.
                    index += i + 1;
                    continue 'outer;
                }
            }
        }
        break;
    }

    Some(index + length)
}

pub fn part_one(input: &str) -> Option<u32> {
    let signal = input.lines().next().unwrap();
    let marker_idx = get_signal_marker(signal, 4).unwrap();

    Some(marker_idx as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let signal = input.lines().next().unwrap();
    let marker_idx = get_signal_marker(signal, 14).unwrap();

    Some(marker_idx as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb".into()), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz".into()), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg".into()), Some(6));
        assert_eq!(
            part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into()),
            Some(10)
        );
        assert_eq!(
            part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into()),
            Some(11)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb".into()), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz".into()), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg".into()), Some(23));
        assert_eq!(
            part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into()),
            Some(29)
        );
        assert_eq!(
            part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into()),
            Some(26)
        );
    }
}
