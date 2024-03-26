use std::{cmp::max, fs, path::Path};

fn main() {
    let file_path: &Path = Path::new("inputs/day2.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    let limit_red = 12;
    let limit_green = 13;
    let limit_blue = 14;

    println!(
        "part 1: {}",
        part1(&contents, &limit_red, &limit_green, &limit_blue)
    );
    println!("part 2: {}", part2(&contents));
}

fn part1(input: &str, limit_red: &u32, limit_green: &u32, limit_blue: &u32) -> u32 {
    let mut id_sum: u32 = 0;

    for line in input.trim().lines().map(|line| line.trim()) {
        let (beginning, end) = line.split_once(':').unwrap();
        let id = beginning
            .trim()
            .split_once(" ")
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap();

        let mut possible = true;
        for round in end.split(";").map(|round| round.trim()) {
            for cube in round.split(",").map(|cube| cube.trim()) {
                let (str_number, color) = cube.split_once(" ").unwrap();
                let number = str_number.parse::<u32>().unwrap();
                let limit = match color {
                    "red" => limit_red,
                    "green" => limit_green,
                    "blue" => limit_blue,
                    _ => panic!("no matching color"),
                };
                if number > *limit {
                    possible = false;
                }
            }
        }
        if possible {
            id_sum += id;
        }
    }

    id_sum
}

fn part2(input: &str) -> u32 {
    let mut power_sum: u32 = 0;

    for line in input.trim().lines().map(|line| line.trim()) {
        let (_, end) = line.split_once(':').unwrap();

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for round in end.split(";").map(|round| round.trim()) {
            for cube in round.split(",").map(|cube| cube.trim()) {
                let (str_number, color) = cube.split_once(" ").unwrap();
                let number = str_number.parse::<u32>().unwrap();
                match color {
                    "red" => max_red = max(number, max_red),
                    "green" => max_green = max(number, max_green),
                    "blue" => max_blue = max(number, max_blue),
                    _ => panic!("no matching color"),
                };
            }
        }

        power_sum += max_red * max_blue * max_green;
    }

    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let limit_red = 12;
        let limit_green = 13;
        let limit_blue = 14;

        assert_eq!(part1(input, &limit_red, &limit_green, &limit_blue), 8);
    }

    #[test]
    fn part1_input2() {
        let file_path: &Path = Path::new("inputs/day2.txt");

        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        let limit_red = 12;
        let limit_green = 13;
        let limit_blue = 14;

        assert_eq!(
            part1(&contents, &limit_red, &limit_green, &limit_blue),
            2541
        );
    }

    #[test]
    fn part2_input1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part2(input), 2286);
    }

    #[test]
    fn part2_input2() {
        let file_path: &Path = Path::new("inputs/day2.txt");

        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part2(&contents), 66016);
    }
}
