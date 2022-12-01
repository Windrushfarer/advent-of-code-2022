use std::iter::Map;
use std::str::Split;

fn count_calories_from(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|line| line.split("\n").map(|s| s.parse::<u32>().unwrap()).sum())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    return count_calories_from(input).into_iter().max();
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = count_calories_from(input);
    calories.sort_unstable_by(|a, b| b.cmp(a));

    let mut iter = calories.iter_mut();
    let result: u32 = *iter.next().unwrap() + *iter.next().unwrap() + *iter.next().unwrap();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
