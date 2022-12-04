use std::collections::{HashSet};

fn calculate_priority(items: Vec<char>) -> usize {
    return items.iter().map(|c| {
        if c.is_lowercase() {
            (*c as u8 - b'a') as usize + 1
        } else {
            (*c as u8 - b'A') as usize + 1 + 26
        }
    }).sum();
}

// [a-z] -> [97-122]
// [A-Z] -> [65-90]
pub fn part_one(input: &str) -> Option<u32> {
    let common_items: Vec<char> = input
        .split("\n")
        .map(|l| {
            let (part1, part2) = l.split_at(l.len() / 2);
            let set1: HashSet<char> = HashSet::from_iter(part1.chars());
            let set2: HashSet<char> = HashSet::from_iter(part2.chars());

            set1.intersection(&set2).next().unwrap().to_owned()
        })
        .collect();

    Some((calculate_priority(common_items)) as u32)
}

fn calculate_badges(input: &str) -> Vec<char> {
    return input
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let mut sets: Vec<HashSet<char>> = group.iter().map(|x| HashSet::from_iter(x.chars())).collect();
            let mut result = sets.pop().unwrap();
            result.retain(|item| {
                sets.iter().all(|set| set.contains(item))
            });

            result
        })
        .flatten()
        .map(|c| c.to_owned())
        .collect();
}

pub fn part_two(input: &str) -> Option<u32> {
    let badges = calculate_badges(input);

    Some(calculate_priority(badges) as u32)
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
