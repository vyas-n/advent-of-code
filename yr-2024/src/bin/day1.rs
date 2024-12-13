use std::fs;
use std::path::Path;

fn const_contents() -> String {
    let contents: &str = "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    contents.to_string()
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day1.txt");

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

fn part1(input: &str) -> u32 {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    input.trim().lines().for_each(|line| {
        let mut iter = line.split_whitespace();

        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    });

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(num1, num2)| num1.abs_diff(*num2))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();

    input.trim().lines().for_each(|line| {
        let mut iter = line.split_whitespace();

        list1.push(iter.next().unwrap().parse::<u32>().unwrap());
        list2.push(iter.next().unwrap().parse::<u32>().unwrap());
    });

    list1
        .iter()
        .map(|num1| (list2.iter().filter(|num2| *num2 == num1).count() as u32) * num1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = const_contents();

        assert_eq!(part1(&contents), 11)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 2176849)
    }

    #[test]
    fn part2_input1() {
        let contents: String = const_contents();

        assert_eq!(part2(&contents), 31)
    }

    #[test]
    fn part2_input2() {
        let contents = file_contents();

        assert_eq!(part2(&contents), 23384288)
    }
}
