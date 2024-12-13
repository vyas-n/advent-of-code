use std::fs;
use std::path::Path;

fn const_contents() -> String {
    let contents: &str = "
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    contents.to_string()
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day2.txt");

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
    input
        .trim()
        .lines()
        .map(|line| {
            // Parse line into report
            line.split_whitespace()
                .into_iter()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| {
            // Filter reports
            let is_increasing = (0..report.len() - 1)
                .map(|index| report[index] < report[index + 1])
                .fold(true, |a, b| a && b);

            let is_decreasing = (0..report.len() - 1)
                .map(|index| report[index] > report[index + 1])
                .fold(true, |a, b| a && b);

            let is_diff_in_range = (0..report.len() - 1)
                .map(|index| {
                    let diff = report[index].abs_diff(report[index + 1]);

                    diff >= 1 && diff <= 3
                })
                .fold(true, |a, b| a && b);

            (is_increasing || is_decreasing) && is_diff_in_range
        })
        .count()
}

fn part2(input: &str) -> i32 {
    todo!("Setup fn part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = const_contents();

        assert_eq!(part1(&contents), 2)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 502)
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
