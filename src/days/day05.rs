use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

struct Page {
    pages: Vec<i32>,
}

impl Page {
    pub fn new(line: &str) -> Self {
        let nums: Vec<i32> = line
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        Self { pages: nums }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    println!("Input: \n{input}");
    let sol1 = 0;
    let sol2 = 0;
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day5_p1() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "143");
    }
}
