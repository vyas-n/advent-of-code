use std::fs;
use std::{path::Path, str::Chars};

fn const_contents() -> String {
    let contents: &str = "
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX";

    contents.to_string()
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day4.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    contents
}

fn main() {
    let contents: String = file_contents();

    println!("Answer: {}", part1(&contents))
}

fn part1(input: &str) -> usize {
    let word = "XMAS";
    // Input parsing
    let word_search: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    #[derive(Clone)]
    enum Direction {
        North,
        NorthEast,
        East,
        SouthEast,
        South,
        SouthWest,
        West,
        NorthWest,
    }

    fn find_word(
        word: &mut Chars,
        row_index: i32,
        col_index: i32,
        direction: Direction,
        word_search: &Vec<Vec<char>>,
    ) -> bool {
        if let Some(current_character) = word.next() {
            if row_index < 0
                || row_index >= word_search.len() as i32
                || col_index < 0
                || col_index >= word_search[row_index as usize].len() as i32
            {
                false
            } else if word_search[row_index as usize][col_index as usize] == current_character {
                let new_row_index = match direction {
                    Direction::NorthEast | Direction::North | Direction::NorthWest => row_index - 1,
                    Direction::East | Direction::West => row_index,
                    Direction::SouthEast | Direction::South | Direction::SouthWest => row_index + 1,
                };

                let new_col_index = match direction {
                    Direction::NorthWest | Direction::West | Direction::SouthWest => col_index - 1,
                    Direction::North | Direction::South => col_index,
                    Direction::NorthEast | Direction::East | Direction::SouthEast => col_index + 1,
                };

                find_word(word, new_row_index, new_col_index, direction, word_search)
            } else {
                false
            }
        } else {
            true
        }
    }

    let mut count = 0;
    for (row_index, row) in word_search.iter().enumerate() {
        for (col_index, _) in row.iter().enumerate() {
            count += [
                Direction::North,
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
            ]
            .into_iter()
            .filter(|direction| {
                find_word(
                    &mut word.chars(),
                    row_index as i32,
                    col_index as i32,
                    direction.clone(),
                    &word_search,
                )
            })
            .count()
        }
    }

    count
}

fn part2(input: &str) -> usize {
    todo!("Setup fn part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = const_contents();

        assert_eq!(part1(&contents), 18)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 2401)
    }

    // #[test]
    // fn part2_input1() {
    //     let contents: String = const_contents();

    //     assert_eq!(part2(&contents), todo!("Setup test output 1 for part2"))
    // }

    // #[test]
    // fn part2_input2() {
    //     let contents = file_contents();

    //     assert_eq!(part2(&contents), todo!("Setup test output 2 for part2"))
    // }
}
