use std::ops::RangeInclusive;

fn parse_range(s: &str) -> RangeInclusive<u32>{
    let (start, end) = s.split_once("-").expect("range is not x-y format");

    return RangeInclusive::new(
        start.parse().expect("start is not a number"),
        end.parse().expect("end is not a number"),
    )
}

fn fully_contains(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    return range1.contains(range2.start()) && range1.contains(range2.end())
}

fn overlaps(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    return range1.contains(range2.start())
        || range1.contains(range2.end())
        || range2.contains(range1.start())
        || range2.contains(range1.end())
}

fn filter_ranges_with<F>(input: &str, predicate: F) -> usize
    where F: Fn(&RangeInclusive<u32>, &RangeInclusive<u32>) -> bool
{
    return input.split("\n")
        .map(|line| {
            let (range1, range2)= line.split_once(",").expect("wrong line format");

            (parse_range(range1), parse_range(range2))
        })
        .filter(|(range1, range2)| predicate(range1, range2))
        .count();
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = filter_ranges_with(input, |range1, range2| {
        fully_contains(range1, range2) || fully_contains(range2, range1)
    });

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = filter_ranges_with(input, |range1, range2| {
        overlaps(range1, range2)
    });

    Some(result as u32)
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
