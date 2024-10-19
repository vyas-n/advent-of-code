use std::{cmp::max, cmp::min, fs, path::Path};

fn main() {
    // let contents: String = "467..114..
    //     ...*......
    //     ..35..633.
    //     ......#...
    //     617*......
    //     .....+.58.
    //     ..592.....
    //     ......755.
    //     ...$.*....
    //     .664.598.."
    //     .to_string();

    let file_path: &Path = Path::new("inputs/day3.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    println!("Sum: {}", part2(&contents))
}

#[derive(Clone, Debug)]
struct Number {
    row_index: usize,
    col_index: usize,
    value: u32,
    log_10_size: usize,
}

fn part1(input: &str) -> u32 {
    // Parse the string into a 2D character grid
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    // Gather all the numbers that appear in the grid
    let mut numbers: Vec<Number> = Vec::new();
    for (row_index, line) in grid.iter().enumerate() {
        let mut sum = 0;
        let mut log_10_size = 0;

        for (col_index, value) in line.iter().enumerate() {
            if value.is_digit(10) {
                sum = sum * 10 + value.to_digit(10).unwrap();
                log_10_size += 1;
            } else if sum != 0 {
                numbers.push(Number {
                    row_index,
                    col_index: col_index - log_10_size,
                    value: sum,
                    log_10_size,
                });
                sum = 0;
                log_10_size = 0;
            }
        }
        if sum != 0 {
            numbers.push(Number {
                row_index,
                col_index: line.len() - log_10_size,
                value: sum,
                log_10_size,
            });
        }
    }

    println!(
        "Total: {}",
        // Sum all the numbers
        numbers
            .iter()
            .map(|number| number.value)
            .fold(0, |a, b| a + b)
    );

    numbers = numbers
        .into_iter()
        .filter(|number| {
            let y_index_range = max(0, number.row_index as i32 - 1)
                ..=min(grid.len() as i32 - 1, number.row_index as i32 + 1);

            let x_index_range = max(0, number.col_index as i32 - 1)
                ..=min(
                    grid[number.row_index as usize].len() as i32 - 1,
                    number.col_index as i32 + number.log_10_size as i32,
                );

            let mut found = false;
            for y_index in y_index_range.clone() {
                for x_index in x_index_range.clone() {
                    let character = grid[y_index as usize][x_index as usize];
                    if (!character.is_digit(10)) && (character != '.') {
                        found = true;
                    }
                }
            }

            // // Debug
            // let debug_map = y_index_range
            //     .clone()
            //     .map(|y_index| {
            //         x_index_range
            //             .clone()
            //             .map(|x_index| grid[y_index as usize][x_index as usize])
            //             .fold(String::new(), |a, b| a + &b.to_string())
            //     })
            //     .fold(String::new(), |a, b| format!("{}\n{}", a, b));
            // println!(
            //     "{:?}\ny_index_range: {:?}\nx_index_range: {:?}\n{}\nFound: {}\n",
            //     number,
            //     y_index_range,
            //     x_index_range,
            //     debug_map.trim(),
            //     if found { "✅" } else { "❌" }
            // );

            // return value
            found
        })
        .collect();

    // Sum the remaining numbers
    numbers
        .iter()
        .map(|number| number.value)
        .fold(0, |a, b| a + b)
}

fn part2(input: &str) -> u32 {
    // Parse the string into a 2D character grid
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    // Gather all the numbers that appear in the grid
    let mut numbers: Vec<Number> = Vec::new();
    for (row_index, line) in grid.iter().enumerate() {
        let mut sum = 0;
        let mut log_10_size = 0;

        for (col_index, value) in line.iter().enumerate() {
            if value.is_digit(10) {
                sum = sum * 10 + value.to_digit(10).unwrap();
                log_10_size += 1;
            } else if sum != 0 {
                numbers.push(Number {
                    row_index,
                    col_index: col_index - log_10_size,
                    value: sum,
                    log_10_size,
                });
                sum = 0;
                log_10_size = 0;
            }
        }
        if sum != 0 {
            numbers.push(Number {
                row_index,
                col_index: line.len() - log_10_size,
                value: sum,
                log_10_size,
            });
        }
    }

    grid.clone()
        .into_iter()
        .enumerate()
        // Check every row
        .map(|(y_index, line)| {
            line.into_iter()
                .enumerate()
                // Filter to only '*' characters (gear)
                .filter(|(_, value)| *value == '*')
                .map(|(x_index, _)| {
                    // Find all the numbers surrounding this point
                    numbers
                        .clone()
                        .into_iter()
                        // Check if the area around the number is this '*' character (gear)
                        .filter(|num| {
                            let y_index_range = max(0, num.row_index as i32 - 1)
                                ..=min(grid.len() as i32 - 1, num.row_index as i32 + 1);

                            let x_index_range = max(0, num.col_index as i32 - 1)
                                ..=min(
                                    grid[num.row_index as usize].len() as i32 - 1,
                                    num.col_index as i32 + num.log_10_size as i32,
                                );

                            y_index_range.contains(&(y_index as i32))
                                && x_index_range.contains(&(x_index as i32))
                        })
                        .collect::<Vec<Number>>()
                })
                // Filter to only lists of size 2
                .filter(|numbers| numbers.len() == 2)
                // Multiply the list values (gears)
                .map(|numbers| numbers.iter().fold(1, |sum, num| sum * num.value))
                // Add up all gears in the column
                .fold(0, |a, b| a + b)
        })
        // Add up all the gears in the row
        .fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
            .to_string();

        assert_eq!(part1(&contents), 4361)
    }

    #[test]
    fn part1_input2() {
        let file_path: &Path = Path::new("inputs/day3.txt");

        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part1(&contents), 531561)
    }

    #[test]
    fn part2_input1() {
        let contents: String = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
            .to_string();

        assert_eq!(part2(&contents), 467835)
    }

    // #[test]
    // fn part2_input2() {
    //     let file_path: &Path = Path::new("inputs/day3.txt");

    //     let contents = fs::read_to_string(&file_path).expect(
    //         format!(
    //             "Should have been able to read the file at file path: {:?}",
    //             &file_path
    //         )
    //         .as_str(),
    //     );

    //     todo!("Setup test input 2 for part2")
    // }
}
