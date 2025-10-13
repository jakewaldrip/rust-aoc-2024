use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
fn create_levels_vec(line: &str) -> Vec<i32> {
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    levels
}

fn evaluate_levels_part_1(levels: &[i32]) -> bool {
    let is_ascending = levels[0] < levels[1];
    for (index, level) in levels.iter().enumerate() {
        if index == levels.len() - 1 {
            break;
        }

        let next_element = levels[index + 1];
        if is_ascending {
            if *level >= next_element || next_element - *level > 3 {
                return false;
            }
        } else if *level <= next_element || *level - next_element > 3 {
            return false;
        }
    }

    true
}

fn deep_evaluate_levels(levels: &[i32]) -> bool {
    todo!()
}

pub fn evaluate_levels_part_2(levels: &[i32]) -> bool {
    let is_ascending = levels[0] < levels[1];

    for (index, level) in levels.iter().enumerate() {
        if index == levels.len() - 1 {
            break;
        }

        let next_element = levels[index + 1];
        if is_ascending {
            if *level >= next_element || next_element - *level > 3 {
                return false;
            }
        } else if *level <= next_element || *level - next_element > 3 {
            return false;
        }
    }

    true
}

pub fn solve(input: &str) -> SolutionPair {
    // p1
    let mut p1_count = 0;
    for line in input.lines() {
        let levels = create_levels_vec(line);
        if evaluate_levels_part_1(&levels) {
            p1_count += 1
        }
    }
    let sol1: u64 = p1_count;

    // p2
    let mut p2_count = 0;
    for line in input.lines() {
        let levels = create_levels_vec(line);
        if evaluate_levels_part_2(&levels) || deep_evaluate_levels(&levels) {
            p2_count += 1;
        }
    }
    let sol2: u64 = p2_count;

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_file_part1() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "2");
    }

    #[test]
    fn test_example_file_part2() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "4");
    }
}
