fn parse_crates(s: &str) -> Vec<Vec<char>> {
    let mut lines =  s.split("\n").collect::<Vec<&str>>();
    lines.reverse();

    let mut lines_iter = lines.iter_mut().peekable();
    let stacks_count = lines_iter.next()
        .unwrap()
        .as_bytes()
        .chunks(4)
        .map(std::str::from_utf8)
        .map(Result::unwrap)
        .map(str::trim)
        .map(|el| el.chars().nth(0).filter(|c| c.is_alphanumeric()).unwrap())
        .count();

    let mut crates: Vec<Vec<char>> = Vec::with_capacity(stacks_count);
    crates.fill(vec![]);

    while lines_iter.peek().is_some() {
        lines_iter.next()
            .unwrap()
            .as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .map(Result::unwrap)
            .map(str::trim)
            .map(|el| el.chars().nth(1).filter(|c| c.is_alphabetic()))
            .enumerate()
            .for_each(|(i, c)| {
                if c.is_some() {
                    let stack = crates.get_mut(i);

                    if stack.is_some() {
                        stack.unwrap().push(c.unwrap());
                    } else {
                        crates.push(vec![c.unwrap()]);
                    }
                }
            })
    }

    return crates
}

fn parse_moves(s: &str) -> Vec<Vec<usize>> {
    return s.split("\n")
        .map(|line| {
            return line.split(" ")
                .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().expect("to be a valid number"))
                .collect::<Vec<usize>>();
        })
        .collect();
}

fn perform_moves(crates: &mut Vec<Vec<char>>, moves: &Vec<Vec<usize>>) {
    moves.iter().for_each(|item| {
        let count = item.get(0).expect("count is available");
        let from_crate = item.get(1).expect("from_crate is available");
        let to_crate = item.get(2).expect("to_crate is available");

        for _ in 0..*count {
            let stack = crates.get_mut(*from_crate - 1)
                .expect("from crate crate exists");
            let item_to_move = stack
                .pop()
                .expect("item to exists");

            crates.get_mut(*to_crate - 1)
                .expect("to crate exists")
                .push(item_to_move)
        }
    });
}

fn perform_group_moves(crates: &mut Vec<Vec<char>>, moves: &Vec<Vec<usize>>) {
    moves.iter().for_each(|item| {
        let count = item.get(0).expect("count is available");
        let from_crate = item.get(1).expect("from_crate is available");
        let to_crate = item.get(2).expect("to_crate is available");
        let stack = crates.get_mut(*from_crate - 1)
            .expect("from crate crate exists");
        let mut slice = stack[stack.len() - count..stack.len()].to_vec();
        stack.drain(stack.len() - count..stack.len());

        crates.get_mut(*to_crate - 1)
            .expect("to crate exists")
            .append(&mut slice);
    });
}

fn get_topline(crates: &Vec<Vec<char>> ) -> String {
    return crates.iter()
        .map(|stack| stack.last())
        .filter(|el| el.is_some())
        .map(|c| c.unwrap())
        .collect::<String>();
}

pub fn part_one(input: &str) -> Option<u32> {
    let (data_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut crates = parse_crates(data_str);
    let moves = parse_moves(moves_str);

    perform_moves(&mut crates, &moves);

    let topline = get_topline(&crates);
    println!("topline: {:?}", topline);
    Some(topline.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (data_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut crates = parse_crates(data_str);
    let moves = parse_moves(moves_str);

    perform_group_moves(&mut crates, &moves);

    let topline = get_topline(&crates);
    println!("topline: {:?}", topline);
    Some(topline.len() as u32)
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
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(3));
    }
}
