use std::{fs, path::Path};

fn main() {
    let file_path: &Path = Path::new("inputs/day1.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    println!("part 1: {}", part1(&contents));
    println!("part 2: {}", part2(&contents));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.trim().lines() {
        let mut first_num = 0;
        let mut last_num = 0;
        for char in line.chars() {
            if let Some(number) = char.to_digit(10) {
                first_num = number;
                break;
            }
        }
        for char in line.chars().rev() {
            if let Some(number) = char.to_digit(10) {
                last_num = number;
                break;
            }
        }
        sum += first_num * 10 + last_num;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut new_input = String::new();

    for line in input.trim().lines() {
        let mut new_line = String::new();

        let clean_line = line.trim();
        for (index, char) in clean_line.char_indices() {
            if char.is_digit(10) {
                new_line += char.to_string().as_ref();
            } else {
                let numbers = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ];
                for (num_index, num) in numbers.into_iter().enumerate() {
                    if index + num.len() - 1 < clean_line.len()
                        && clean_line[index..index + num.len()] == *num
                    {
                        new_line += format!("{}", num_index + 1).as_ref();
                    }
                }
            }
        }
        new_input += format!("{}\n", new_line).as_ref();
    }

    let mut sum = 0;
    for line in new_input.trim().lines() {
        let mut first_num = 0;
        let mut last_num = 0;
        for char in line.chars() {
            if let Some(number) = char.to_digit(10) {
                first_num = number;
                break;
            }
        }
        for char in line.chars().rev() {
            if let Some(number) = char.to_digit(10) {
                last_num = number;
                break;
            }
        }
        sum += first_num * 10 + last_num;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part1_input2() {
        let file_path: &Path = Path::new("inputs/day1.txt");

        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part1(&contents), 53921);
    }

    #[test]
    fn part2_input1() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }

    #[test]
    fn part2_input2() {
        let file_path: &Path = Path::new("inputs/day1.txt");

        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part2(&contents), 54676);
    }
}
