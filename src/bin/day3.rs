use std::{cmp::max, cmp::min, fs, path::Path};

fn main() {
    let file_path: &Path = Path::new("inputs/day3.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    println!("Sum: {}", part1(&contents))

    // let limit_red = 12;
    // let limit_green = 13;
    // let limit_blue = 14;

    // println!(
    //     "part 1: {}",
    //     part1(&contents, &limit_red, &limit_green, &limit_blue)
    // );
    // println!("part 2: {}", part2(&contents));
}

fn part1(input: &str) -> u32 {
    #[derive(Debug)]
    struct Number {
        row_index: usize,
        col_index: usize,
        value: u32,
        log_10_size: usize,
    }

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
        // Sum the remaining numbers
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

            // Debug
            // let mut debug_map = y_index_range
            //     .clone()
            //     .map(|y_index| {
            //         let sup = x_index_range
            //             .clone()
            //             .map(|x_index| grid[y_index as usize][x_index as usize])
            //             .fold(String::new(), |a, b| a + &b.to_string());

            //         sup
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
    todo!("Setup fn part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let map = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";

        assert_eq!(part1(&map), 4361)
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

    // #[test]
    // fn part2_input1() {
    //     todo!("Setup test input 1 for part2")
    // }

    // #[test]
    // fn part2_input2() {
    //     let file_path: &Path = Path::new("inputs/day#.txt");

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
