use crate::{
    Solution, SolutionPair,
    utils::grid::{ALL_DIRECTIONS, Directions, Grid, Point},
};

///////////////////////////////////////////////////////////////////////////////

// Could optimize visited hashmap in to avoid searching letters part of existing solutions
struct WordSearch {
    grid: Grid,
}

impl WordSearch {
    pub const WORD: &str = "XMAS";

    fn get_next_match_with_direction(
        &self,
        curr_point: &Point,
        next_letter: &u8,
        direction: &Directions,
    ) -> Option<Point> {
        // if direction provided, check if theres the next letter in that direction
        if let Some(point_in_direction) = self.grid.get_point_in_direction(curr_point, direction)
            && &point_in_direction.value == next_letter
        {
            return Some(point_in_direction);
        }

        None
    }

    fn get_next_letter(letter: &u8) -> Option<u8> {
        match letter {
            b'X' => Some(b'M'),
            b'M' => Some(b'A'),
            b'A' => Some(b'S'),
            b'S' => None,
            _ => panic!("Unsupported letter in input"),
        }
    }

    fn evaluate_matching_word_p1(&self, point: &Point) -> i32 {
        // Always start with X, otherwise we don't have a match
        if point.value != b'X' {
            return 0;
        }

        let mut curr_point: Point;
        let mut count = 0;

        for direction in ALL_DIRECTIONS {
            curr_point = *point;
            for letter in Self::WORD.bytes() {
                if let Some(next_letter) = Self::get_next_letter(&letter) {
                    if let Some(next_match_point) =
                        self.get_next_match_with_direction(&curr_point, &next_letter, &direction)
                    {
                        // Move cursor to current point
                        curr_point = next_match_point;
                    } else {
                        break;
                    }
                } else {
                    count += 1;
                }
            }
        }

        count
    }

    fn evaluate_matching_word_p2(&self, point: &Point) -> i32 {
        // looking for X shape of 2 mas words, forward or backwards
        // approach: get all points for the x shape, split into 2 words
        // check if those words are 1. all Some(), and 2. spell mas or sam
        todo!()
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let grid = Grid::new(input);
    println!("{grid}");
    let word_search = WordSearch { grid };

    let mut sol1 = 0;
    for point in word_search.grid.iter_2d() {
        sol1 += word_search.evaluate_matching_word_p1(&point)
    }

    let mut sol2 = 0;
    for point in word_search.grid.iter_2d() {
        sol2 += word_search.evaluate_matching_word_p2(&point)
    }

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_p1() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "18");
    }

    #[test]
    fn test_example_input_4_0_right() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let grid = Grid::new(input);
        let word_search = WordSearch { grid };
        let found = word_search.evaluate_matching_word_p1(&Point {
            row: 4,
            col: 0,
            value: b'X',
        });
        assert_eq!(found, 1);
    }

    #[test]
    fn test_example_input_double_count() {
        let input = "XMASXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nSSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let grid = Grid::new(input);
        let word_search = WordSearch { grid };
        let found = word_search.evaluate_matching_word_p1(&Point {
            row: 0,
            col: 0,
            value: b'X',
        });
        assert_eq!(found, 2);
    }

    #[test]
    fn test_example_input_p2_day4() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "9");
    }
}
