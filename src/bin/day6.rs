use std::path::Path;
use std::{fmt::Display, fs};

fn const_contents() -> String {
    let contents: String = "Time:      7  15   30
    Distance:  9  40  200"
        .to_string();

    contents
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day6.txt");

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

struct Race {
    time: u64,
    record_dist: u64,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m in {}s", self.record_dist, self.time)
    }
}

fn part1(input: &str) -> u64 {
    // Parse input into a list of races
    let mut input_iter = input.trim().lines().map(|line| line.trim());
    let times = input_iter
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap());
    let distances = input_iter
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap());
    let races = times
        .zip(distances)
        .map(|(time, record_dist)| Race { time, record_dist });

    // Debug
    println!("Races:");
    races.clone().for_each(|race| {
        println!("- {}", race);
    });
    println!("");

    races
        .map(|race| {
            // Find the first time + dist that can win this race
            let (time, _) = (0..race.time)
                .map(|time| {
                    let dist = time * (race.time - time);

                    (time, dist)
                })
                .find(|(_, dist)| *dist > race.record_dist)
                .unwrap();

            (race.time - time) - time + 1
        })
        .fold(1, |a, b| a * b) // Return the product of all the possible ways to win each race
}

fn part2(input: &str) -> u64 {
    // Parse input into a list of races
    let mut input_iter = input.trim().lines().map(|line| line.trim());
    let time = input_iter
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let record_dist = input_iter
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let race = Race { time, record_dist };

    // Debug
    println!("Race: {}", race);
    println!("");

    // Find the first time + dist that can win this race
    let (time, _) = (0..race.time)
        .map(|time| {
            let dist = time * (race.time - time);

            (time, dist)
        })
        .find(|(_, dist)| *dist > race.record_dist)
        .unwrap();

    (race.time - time) - time + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = const_contents();

        assert_eq!(part1(&contents), 288)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 449820)
    }

    #[test]
    fn part2_input1() {
        let contents: String = const_contents();

        assert_eq!(part2(&contents), 71503)
    }

    #[test]
    fn part2_input2() {
        let contents = file_contents();

        assert_eq!(part2(&contents), 42250895)
    }
}
