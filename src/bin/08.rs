use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Visible {
    west: bool,
    north: bool,
    east: bool,
    south: bool,
}

impl Index<&str> for Visible {
    type Output = bool;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "west" => &self.west,
            "north" => &self.north,
            "east" => &self.east,
            "south" => &self.south,
            _ => panic!("Invalid direction {index}"),
        }
    }
}

impl IndexMut<&str> for Visible {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "west" => &mut self.west,
            "north" => &mut self.north,
            "east" => &mut self.east,
            "south" => &mut self.south,
            _ => panic!("Invalid direction {index}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tree {
    visible: Visible,
    height: u8,
}

impl Tree {
    fn new(value: u8) -> Self {
        Tree {
            visible: Visible {
                west: false,
                north: false,
                east: false,
                south: false,
            },
            height: value,
        }
    }
}

fn set_visible(trees: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    let mut new_trees = trees.clone();
    let mut max_from_north = vec![0; trees[0].len()];
    let mut max_from_south = vec![0; trees[0].len()];
    for (row_idx, row) in trees.iter().enumerate() {
        // From west
        let mut max_height_from_west = 0;
        let mut max_height_from_east = 0;
        for (tree_idx, tree) in row.iter().enumerate() {
            let west_tree = tree;
            let east_tree = &row[row.len() - tree_idx - 1];
            if tree_idx == 0 {
                // WEST
                new_trees[row_idx][tree_idx].visible["west"] = true;
                max_height_from_west = west_tree.height;

                //EAST
                new_trees[row_idx][row.len() - tree_idx - 1].visible["east"] = true;
                max_height_from_east = east_tree.height;
            } else {
                //WEST
                if west_tree.height > max_height_from_west {
                    new_trees[row_idx][tree_idx].visible["west"] = true;
                    max_height_from_west = west_tree.height;
                }

                //EAST
                if east_tree.height > max_height_from_east {
                    new_trees[row_idx][row.len() - tree_idx - 1].visible["east"] = true;
                    max_height_from_east = east_tree.height;
                }
            }

            let north_tree = tree;
            let south_tree = &trees[trees.len() - row_idx - 1][tree_idx];
            if row_idx == 0 {
                // NORTH
                new_trees[row_idx][tree_idx].visible["north"] = true;
                max_from_north[tree_idx] = north_tree.height;

                // SOUTH
                new_trees[trees.len() - row_idx - 1][tree_idx].visible["south"] = true;
                max_from_south[tree_idx] = south_tree.height;
            } else {
                //NORTH
                if north_tree.height > max_from_north[tree_idx] {
                    new_trees[tree_idx][row_idx].visible["north"] = true;
                    max_from_north[tree_idx] = north_tree.height;
                }

                //SOUTH
                if south_tree.height > max_from_south[tree_idx] {
                    new_trees[trees.len() - row_idx - 1][tree_idx].visible["south"] = true;
                    max_from_south[tree_idx] = south_tree.height;
                }
            }
        }
    }
    new_trees
}

fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    let parsed_trees: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree::new(c.to_digit(10).unwrap() as u8))
                .collect()
        })
        .collect();

    let trees = set_visible(parsed_trees);

    trees
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = parse_input(input);

    let num_visible = trees
        .iter()
        .flatten()
        .filter(|t| t.visible.east || t.visible.north || t.visible.south || t.visible.west)
        .count();

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
