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
        direction: &Option<Directions>,
    ) -> Option<(Point, Directions)> {
        if let Some(direction) = direction {
            // if direction provided, check if theres the next letter in that direction
            if let Some(point_in_direction) =
                self.grid.get_point_in_direction(curr_point, direction)
                && &point_in_direction.value == next_letter
            {
                return Some((point_in_direction, *direction));
            }
        } else {
            // if direciton not provided, check all 8 directions for the next letter
            for dir in ALL_DIRECTIONS {
                if let Some(point_in_direction) = self.grid.get_point_in_direction(curr_point, &dir)
                    && &point_in_direction.value == next_letter
                {
                    return Some((point_in_direction, dir));
                }
            }
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

    fn evaluate_matching_word(&self, point: &Point) -> bool {
        // Always start with X, otherwise we don't have a match
        if point.value != b'X' {
            return false;
        }

        let mut curr_point: Point = *point;
        let mut curr_direction = None;
        for letter in Self::WORD.bytes() {
            if let Some(next_letter) = Self::get_next_letter(&letter) {
                if let Some((next_match_point, next_match_direction)) =
                    self.get_next_match_with_direction(&curr_point, &next_letter, &curr_direction)
                {
                    // Set direction if we haven't already
                    if curr_direction.is_none() {
                        curr_direction = Some(next_match_direction);
                    }

                    // Move cursor to current point
                    curr_point = next_match_point;
                } else {
                    return false;
                }
            }
        }

        println!(
            "Found match, starts on {:?}, ends on {:?}, goes direction: {:?}",
            point, curr_point, curr_direction
        );
        true
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let grid = Grid::new(input);
    println!("{grid}");
    let word_search = WordSearch { grid };

    let mut count = 0;
    for point in word_search.grid.iter_2d() {
        if word_search.evaluate_matching_word(&point) {
            count += 1;
        }
    }
    let sol1 = count;

    let sol2 = 0;
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

    // #[test]
    // fn test_example_input_p1() {
    //     let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
    //     let p1_result = format!("{p1}");
    //     assert_eq!(p1_result, "18");
    // }
}
