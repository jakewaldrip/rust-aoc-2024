use regex::{Match, Regex};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
pub fn get_operands_from_match(input: &str) -> (i32, i32) {
    let re = Regex::new(r"\d+").unwrap();
    let operands: Vec<i32> = re
        .find_iter(input)
        .map(|mat| {
            let operand_str: &str = mat.into();
            operand_str.parse().unwrap()
        })
        .collect();

    (operands[0], operands[1])
}

pub fn is_do_active(mat: Match<'_>, dos_and_donts_map: &[(usize, bool)]) -> bool {
    let mult_start = mat.start();

    if mult_start < dos_and_donts_map[0].0 {
        return true;
    }

    let closest_index_after = binary_search(dos_and_donts_map, mult_start);
    dos_and_donts_map[closest_index_after - 1].1
}

pub fn binary_search(slice: &[(usize, bool)], target: usize) -> usize {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;

        if slice[mid].0 < target {
            low = mid + 1;
        } else if slice[mid].0 > target {
            high = mid;
        } else {
            return mid;
        }
    }

    low
}

pub fn solve(input: &str) -> SolutionPair {
    // p1
    let re = Regex::new(r"mul\(\d{1,},\d{1,}\)").unwrap();
    let sol1: i32 = re
        .find_iter(input)
        .map(|mat| {
            let (a, b) = get_operands_from_match(mat.into());
            a * b
        })
        .sum();

    let re_dos = Regex::new(r"do\(\)").unwrap();
    let re_donts = Regex::new(r"don't\(\)").unwrap();

    let all_dos: Vec<Match<'_>> = re_dos.find_iter(input).collect();
    let all_donts: Vec<Match<'_>> = re_donts.find_iter(input).collect();
    let mut dos_and_donts_map: Vec<(usize, bool)> = [&all_dos[..], &all_donts[..]]
        .concat()
        .iter()
        .map(|mat| {
            let value = match mat.as_str() {
                r"do()" => true,
                r"don't()" => false,
                _ => panic!("Not a do or a dont in the map creation"),
            };

            let start = mat.start();
            (start, value)
        })
        .collect();

    // p2
    dos_and_donts_map.sort_by_key(|&(start, _)| start);
    let sol2: i32 = re
        .find_iter(input)
        .map(|mat| {
            if is_do_active(mat, &dos_and_donts_map) {
                let (a, b) = get_operands_from_match(mat.into());
                a * b
            } else {
                0
            }
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do()_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "161");
    }

    #[test]
    fn test_example_input_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "48");
    }

    #[test]
    fn test_get_operands() {
        let input = "mul(2, 44)";
        let (a, b) = get_operands_from_match(input);
        assert_eq!(a, 2);
        assert_eq!(b, 44);
    }

    #[test]
    fn test_bin_search() {
        let search_arr = vec![(0, true), (1, false), (5, true), (9, true), (12, false)];
        let target = 9;

        let result = binary_search(&search_arr, target);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_bin_search_not_exact() {
        let search_arr = vec![(0, true), (1, false), (5, true), (9, true), (12, false)];
        let target = 3;

        let result = binary_search(&search_arr, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_bin_search_not_exact_longer() {
        let search_arr = vec![
            (0, true),
            (1, false),
            (5, true),
            (9, true),
            (12, false),
            (15, false),
            (20, false),
            (30, false),
            (35, true),
            (51, true),
        ];
        let target = 36;

        let result = binary_search(&search_arr, target);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_bin_search_not_exact_first_element() {
        let search_arr = vec![(2, true), (4, false), (5, true), (9, true), (51, true)];
        let target = 1;

        let result = binary_search(&search_arr, target);
        assert_eq!(result, 0);
    }
}
