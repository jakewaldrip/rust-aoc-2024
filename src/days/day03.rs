use regex::Regex;

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

pub fn solve(input: &str) -> SolutionPair {
    // p1
    let re = Regex::new(r"mul\(\d{1,},\d{1,}\)").unwrap();
    let all_matches = re.find_iter(input);
    let sol1: i32 = re
        .find_iter(input)
        .map(|mat| {
            let (a, b) = get_operands_from_match(mat.into());
            a * b
        })
        .sum();

    // p2: TODO
    let all_mults = re.find_iter(input);
    let sol2 = 0;
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "161");
    }

    #[test]
    fn test_example_input_p2() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "48");
    }

    #[test]
    fn test_get_operands() {
        let input = "mul(2, 44)";
        let (a, b) = get_operands_from_match(input);
        assert_eq!(a, 2);
        assert_eq!(b, 44);
    }
}
