use std::fs;
use std::path::Path;

use regex::Regex;

fn const_contents() -> String {
    let contents: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    contents.to_string()
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day3.txt");

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

struct Mul {
    num1: i32,
    num2: i32,
}

impl From<&str> for Mul {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"[0-9]+").unwrap();
        let mut iter = re.find_iter(value);

        Mul {
            num1: iter.next().unwrap().as_str().parse().unwrap(),
            num2: iter.next().unwrap().as_str().parse().unwrap(),
        }
    }
}

fn part1(input: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();

    re.find_iter(input)
        .map(|substring| Mul::from(substring.as_str()))
        .map(|mul| mul.num1 * mul.num2)
        .sum()
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

        assert_eq!(part1(&contents), 161)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 184122457)
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
