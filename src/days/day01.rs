use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn subtract_vectors(left: &[i32], right: &[i32]) -> Vec<i32> {
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| (*x - *y).abs())
        .collect()
}

fn build_frequency_map(list: &[i32]) -> HashMap<&i32, i32> {
    let mut map: HashMap<&i32, i32> = HashMap::new();
    for num in list {
        *map.entry(num).or_insert(0) += 1;
    }

    map
}

pub fn solve(input: &str) -> SolutionPair {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line.len() >= 13 {
            let left_num = &line[0..5];
            let right_num = &line[8..13];

            left.push(left_num.parse::<i32>().unwrap());
            right.push(right_num.parse::<i32>().unwrap());
        }
    }

    // p1
    left.sort();
    right.sort();
    let distances = subtract_vectors(&left, &right);
    let sol1: i32 = distances.iter().sum();

    // p2
    let frequency_map = build_frequency_map(&right);
    let sol2: i32 = left
        .iter()
        .map(|x| {
            let mult = frequency_map.get(x).unwrap_or(&0);
            x * mult
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_short_list_in_order() {
        let (p1, _) = solve("10000   20000\n20000   30000\n30000   40000");
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "30000");
    }

    #[test]
    fn test_p1_short_list_out_of_order() {
        let (p1, _) = solve("20000   20000\n10000   30000\n30000   40000");
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "30000");
    }

    #[test]
    fn test_subtract_vectors() {
        let left = vec![5, 10, 15];
        let right = vec![1, 2, 20];
        let result = subtract_vectors(&left, &right);
        assert_eq!(result, vec![4, 8, 5]);
    }
}
