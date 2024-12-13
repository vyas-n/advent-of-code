use std::fs;
use std::path::Path;

use regex::Regex;

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

    println!("Answer: {}", part2(&contents))
}

fn part1(input: &str) -> i32 {
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

    let re: Regex = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();

    re.find_iter(input)
        .map(|substring| Mul::from(substring.as_str()))
        .map(|mul| mul.num1 * mul.num2)
        .sum()
}

fn part2(input: &str) -> i32 {
    enum Instruction {
        Mul { num1: i32, num2: i32 },
        Do { enabled: bool },
    }

    impl From<&str> for Instruction {
        fn from(value: &str) -> Self {
            if value.starts_with("mul") {
                let re = Regex::new(r"[0-9]+").unwrap();
                let mut iter = re.find_iter(value);

                Instruction::Mul {
                    num1: iter.next().unwrap().as_str().parse().unwrap(),
                    num2: iter.next().unwrap().as_str().parse().unwrap(),
                }
            } else if value.starts_with("don't") {
                Instruction::Do { enabled: false }
            } else if value.starts_with("do") {
                Instruction::Do { enabled: true }
            } else {
                panic!("This string shouldn't have been parsed.")
            }
        }
    }

    struct Answer {
        enabled: bool,
        sum: i32,
    }

    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)|don\'t\(\)|do\(\)").unwrap();

    re.find_iter(input)
        .map(|substring| {
            println!("{}", substring.as_str());
            let inst = Instruction::from(substring.as_str());

            inst
        })
        .fold(
            Answer {
                enabled: true,
                sum: 0,
            },
            |answer, instruction| match instruction {
                Instruction::Mul { num1, num2 } => {
                    if answer.enabled {
                        Answer {
                            enabled: answer.enabled,
                            sum: answer.sum + (num1 * num2),
                        }
                    } else {
                        answer
                    }
                }
                Instruction::Do { enabled } => Answer {
                    enabled,
                    sum: answer.sum,
                },
            },
        )
        .sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

        assert_eq!(part1(&contents), 161)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 184122457)
    }

    #[test]
    fn part2_input1() {
        let contents: String =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

        assert_eq!(part2(&contents), 48)
    }

    #[test]
    fn part2_input2() {
        let contents = file_contents();

        assert_eq!(part2(&contents), 107862689)
    }
}
