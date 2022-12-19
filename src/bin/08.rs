fn get_visible_trees(trees: Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let max_length = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let line_max_length = row.len() - 1;
            row.iter()
                .enumerate()
                .map(|(col_idx, tree)| {
                    row_idx == 0
                        || row_idx == max_length
                        || col_idx == 0
                        || col_idx == line_max_length
                })
            .collect()
        })
    .collect();

    for row in 0..trees.len() {
        let mut max_west = 0;
        let mut max_east = 0;
        let mut max_north = 0;
        let mut max_south = 0;

        for col in 0..trees[0].len() {
            let east_col_idx = trees[0].len() - 1 - col;
            let south_row_idx = trees.len() - 1 - row;
            if row == 0 {
                max_west = trees[row][col];
                max_east = trees[row][east_col_idx];
                max_north = trees[col][row];
                max_south = trees[col][south_row_idx];
            } else {
                // WEST
                if trees[row][col] > max_west {
                    max_west = trees[row][col];
                    visible_trees[row][col] = true;
                }

                // EAST
                if trees[row][east_col_idx] > max_east {
                    max_east = trees[row][east_col_idx];
                    visible_trees[row][east_col_idx] = true;
                }

                // NORTH
                if trees[col][row] > max_north {
                    max_north = trees[col][row];
                    visible_trees[col][row] = true;
                }

                // SOUTH
                if trees[col][south_row_idx] > max_south {
                    max_south = trees[col][south_row_idx];
                    visible_trees[col][south_row_idx] = true;
                }
            }
        }
    }

    visible_trees
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let parsed_trees: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
    .collect();

    parsed_trees
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = parse_input(input);
    let visible_trees = get_visible_trees(trees);

    let num_visible = visible_trees.iter().flatten().filter(|&&tree| tree).count();

    Some(num_visible as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
