fn parse_input(input: &str) -> Vec<Vec<usize>> {
    return input
        .split("\n")
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = parse_input(input);
    let mut visible_count = (trees.len() * 2) + (trees[0].len() * 2) - 4;

    for row in 1..trees.len() - 1 {
        for col in 1..trees[0].len() - 1 {
            let height = trees[row][col];
            let mut top_visible = true;
            let mut left_visible = true;
            let mut right_visible = true;
            let mut bottom_visible = true;

            // To the right
            for i in (col + 1)..trees[row].len() {
                if trees[row][i] >= height {
                    right_visible = false;
                    break;
                }
            }

            // To the bottom
            for i in (row + 1)..trees.len() {
                if trees[i][col] >= height {
                    bottom_visible = false;
                    break;
                }
            }


            // To the left
            for i in (0..col).rev() {
                if trees[row][i] >= height {
                    top_visible = false;
                    break;
                }
            }

            // To the bottom
            for i in (0..row).rev() {
                if trees[i][col] >= height {
                    left_visible = false;
                    break;
                }
            }

            if top_visible || left_visible || right_visible || bottom_visible {
                visible_count += 1;
            }
        }
    }

    println!("{:?}", visible_count);

    Some(visible_count as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let size = grid.len();
    let mut scores: Vec<Vec<i32>> = Vec::from_iter(grid.iter().map(|row| {
        Vec::from_iter(row.iter().map(|_| 1))
    }));

    let mut stack: Vec<usize> = vec![];

    // Calculate visibility for rows
    for row in 0..size {
        for col in 0..size {
            while !stack.is_empty() && grid[row][col] >= grid[row][*(stack.last().unwrap())] {
                let i = stack.pop().unwrap();
                scores[row][i] *= col as i32 - i as i32
            }
            stack.push(col);
        }

        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            scores[row][i] *= size as i32 - i as i32 - 1;
        }

        for col in (0..=size - 1).rev() {
            while !stack.is_empty() && grid[row][col] >= grid[row][*(stack.last().unwrap())] {
                let i = stack.pop().unwrap();
                scores[row][i] *= i as i32 - col as i32;
            }
            stack.push(col);
        }

        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            scores[row][i] *= i as i32;
        }
    }

    // Calculate visibility for cols
    for col in 0..size {
        for row in 0..size {
            while !stack.is_empty() && grid[row][col] >= grid[*(stack.last().unwrap())][col] {
                let i = stack.pop().unwrap();
                scores[i][col] *= row as i32 - i as i32;
            }
            stack.push(row);
        }
        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            scores[i][col] *= size as i32 - i as i32 - 1;
        }

        for row in (0..=size-1).rev() {
            while !stack.is_empty() && grid[row][col] >= grid[*(stack.last().unwrap())][col] {
                let i = stack.pop().unwrap();
                scores[i][col] *= i as i32 - row as i32
            }
            stack.push(row);
        }
        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            scores[i][col] *= i as i32;
        }
    }

    let max_score = scores.iter().flatten().max().unwrap();

    Some(*max_score as u32)
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
        assert_eq!(part_two(&input), Some(8));
    }
}
