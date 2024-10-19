use std::{cmp::max, fs, path::Path};

fn main() {
    let contents: String = todo!("Setup test input 1 for part1");

    // let file_path: &Path = Path::new("inputs/day#.txt");

    // let contents = fs::read_to_string(&file_path).expect(
    //     format!(
    //         "Should have been able to read the file at file path: {:?}",
    //         &file_path
    //     )
    //     .as_str(),
    // );

    println!("Answer: {}", part1(&contents))
}

fn part1(input: &str) -> u32 {
    todo!("Setup fn part1")
}

fn part2(input: &str) -> u32 {
    todo!("Setup fn part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_input1() {
    //     let contents: String = todo!("Setup test input 1 for part1");

    //     assert_eq!(part1(&contents), todo!("Setup test output 1 for part1"))
    // }

    // #[test]
    // fn part1_input2() {
    //     let file_path: &Path = Path::new("inputs/day#.txt");

    //     let contents = fs::read_to_string(&file_path).expect(
    //         format!(
    //             "Should have been able to read the file at file path: {:?}",
    //             &file_path
    //         )
    //         .as_str(),
    //     );

    //     todo!("Setup test input 2 for part1")
    // }

    // #[test]
    // fn part2_input1() {
    //     let contents: String = todo!("Setup test input 1 for part2");

    //     assert_eq!(part1(&contents), todo!("Setup test output 1 for part2"))
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
