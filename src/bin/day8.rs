use std::{collections::hash_map, path::Path};
use std::{collections::HashMap, fs};

fn const_contents1() -> String {
    let contents: &str = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    contents.to_string()
}

fn const_contents2() -> String {
    let contents: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    contents.to_string()
}

fn const_contents3() -> String {
    let contents: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    contents.to_string()
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day8.txt");

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

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

struct Map {
    hash_map: HashMap<String, (String, String)>,
}

impl Map {
    fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    fn add_navigation(&mut self, starting_node: &str, left_node: &str, right_node: &str) {
        let old_value = self.hash_map.insert(
            starting_node.to_string(),
            (left_node.to_string(), right_node.to_string()),
        );

        if old_value != None {
            panic!("navigation overrode existing value!")
        }
    }

    fn lookup(&self, current_node: &str, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.hash_map[current_node].0,
            Direction::Right => &self.hash_map[current_node].1,
        }
    }
}

fn part1(input: &str) -> u64 {
    // Parse Input
    let mut input_iter = input.trim().lines().into_iter();
    let direction_iter = input_iter
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|character| match character {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        })
        .cycle();
    // Skip empty line
    input_iter.next();
    let mut map = Map::new();
    input_iter.for_each(|line| {
        let (starting_node, navigation) = line
            .trim()
            .strip_suffix(")")
            .unwrap()
            .split_once(" = (")
            .unwrap();
        let (left_node, right_node) = navigation.split_once(", ").unwrap();

        map.add_navigation(starting_node, left_node, right_node);
    });

    // Output
    let mut current_position = "AAA".to_string();
    direction_iter
        .enumerate()
        .map(|(index, direction)| (index as u64 + 1, direction))
        .find(|(_, direction)| {
            current_position = map.lookup(&current_position, &direction).to_string();

            current_position == "ZZZ"
        })
        .unwrap()
        .0
}

fn part2(input: &str) -> u64 {
    // Parse Input
    let mut input_iter = input.trim().lines().into_iter();
    let direction_iter = input_iter
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|character| match character {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        })
        .cycle();
    // Skip empty line
    input_iter.next();
    let mut map = Map::new();
    input_iter.for_each(|line| {
        let (starting_node, navigation) = line
            .trim()
            .strip_suffix(")")
            .unwrap()
            .split_once(" = (")
            .unwrap();
        let (left_node, right_node) = navigation.split_once(", ").unwrap();

        map.add_navigation(starting_node, left_node, right_node);
    });

    // Output
    todo!("Setup fn part2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = const_contents1();

        assert_eq!(part1(&contents), 2)
    }

    #[test]
    fn part1_input2() {
        let contents: String = const_contents2();

        assert_eq!(part1(&contents), 6)
    }

    #[test]
    fn part1_input3() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 16697)
    }

    #[test]
    fn part2_input1() {
        let contents: String = const_contents3();

        assert_eq!(part2(&contents), 6)
    }

    // #[test]
    // fn part2_input1() {
    //     let contents: String = const_contents2();

    //     assert_eq!(part2(&contents), todo!("Setup test output 1 for part2"))
    // }

    // #[test]
    // fn part2_input3() {
    //     let contents = file_contents();

    //     assert_eq!(part2(&contents), todo!("Setup test output 2 for part2"))
    // }
}
