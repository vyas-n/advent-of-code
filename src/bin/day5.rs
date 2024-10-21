use std::{fmt, fs, iter::Map, path::Display};
use std::{io::Read, path::Path};

fn main() {
    // let contents: String = "seeds: 79 14 55 13

    //     seed-to-soil map:
    //     50 98 2
    //     52 50 48

    //     soil-to-fertilizer map:
    //     0 15 37
    //     37 52 2
    //     39 0 15

    //     fertilizer-to-water map:
    //     49 53 8
    //     0 11 42
    //     42 0 7
    //     57 7 4

    //     water-to-light map:
    //     88 18 7
    //     18 25 70

    //     light-to-temperature map:
    //     45 77 23
    //     81 45 19
    //     68 64 13

    //     temperature-to-humidity map:
    //     0 69 1
    //     1 0 69

    //     humidity-to-location map:
    //     60 56 37
    //     56 93 4"
    //     .to_string();

    let file_path: &Path = Path::new("inputs/day5.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    println!("Answer: {}", part2(&contents))
}

#[derive(Clone, Copy, Debug)]
struct MapRule {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl fmt::Display for MapRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.destination_range_start < self.source_range_start {
            write!(
                f,
                "{}..{} -{}",
                self.source_range_start,
                self.source_range_start + self.range_length,
                self.source_range_start - self.destination_range_start
            )
        } else {
            write!(
                f,
                "{}..{} +{}",
                self.source_range_start,
                self.source_range_start + self.range_length,
                self.destination_range_start - self.source_range_start
            )
        }
    }
}

fn part1(input: &str) -> i64 {
    // Split the puzzle into sections by double new line
    let mut puzzle_sections = input.trim().split("\n\n").map(|section| section.trim());

    // Parse Seeds into iterator
    let seeds = puzzle_sections
        .next()
        .unwrap()
        .trim()
        .strip_prefix("seeds:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<i64>().unwrap());

    // Debug
    println!("Seeds: {:?}", seeds.clone().collect::<Vec<i64>>());

    // Parse sections into maps
    let maps = puzzle_sections.map(|section| {
        // Parse each Section into a map index
        section.lines().skip(1).map(|line| {
            // Parse each line into a tuple
            let mut nums = line
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap());

            MapRule {
                destination_range_start: nums.next().unwrap(),
                source_range_start: nums.next().unwrap(),
                range_length: nums.next().unwrap(),
            }
        })
    });

    // Debug
    maps.clone().enumerate().for_each(|(map_index, map)| {
        println!("Map {}:", map_index);
        map.clone().for_each(|rule| println!("- {}", rule));
        println!("");
    });

    // Process each seed
    seeds
        .map(|mut seed| {
            maps.clone().for_each(|map| {
                // If a rule matches, then transform the value
                if let Some(maprule) = map
                    .filter(|maprule| {
                        maprule.source_range_start <= seed
                            && seed < maprule.source_range_start + maprule.range_length
                    })
                    .next()
                {
                    seed = seed - maprule.source_range_start + maprule.destination_range_start;
                }
                // Otherwise leave the value as-is
            });

            seed
        })
        // Return the lowest number (location)
        .min()
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
struct SeedRange {
    start_index: i64,
    length: i64,
}

impl fmt::Display for SeedRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}..{}",
            self.start_index,
            self.start_index + self.length
        )
    }
}

fn part2(input: &str) -> i64 {
    // Split the puzzle into sections by double new line
    let mut puzzle_sections = input.trim().split("\n\n").map(|section| section.trim());

    // Parse Line into numbers iterator
    let mut seeds_input = puzzle_sections
        .next()
        .unwrap()
        .trim()
        .strip_prefix("seeds:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse::<i64>().unwrap());
    // Parse numbers into seeds iterator
    let mut seeds: Vec<SeedRange> = Vec::new();
    while let Some(start_index) = seeds_input.next() {
        let length = seeds_input.next().unwrap();

        seeds.push(SeedRange {
            start_index,
            length,
        });
    }

    // Debug
    println!("Seeds:");

    seeds.clone().iter().for_each(|seed| println!("- {}", seed));

    // Sort each seed by start_index
    seeds.sort_by(|a, b| a.start_index.cmp(&b.start_index));

    // Parse sections into maps
    let maps = puzzle_sections.map(|section| {
        // Parse each Section into a map index
        let mut map = section
            .lines()
            .skip(1)
            .map(|line| {
                // Parse each line into a tuple
                let mut nums = line
                    .trim()
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap());

                MapRule {
                    destination_range_start: nums.next().unwrap(),
                    source_range_start: nums.next().unwrap(),
                    range_length: nums.next().unwrap(),
                }
            })
            .collect::<Vec<MapRule>>();

        // Sort each map index by source_range_start
        map.sort_by(|seed_a, seed_b| seed_a.source_range_start.cmp(&seed_b.source_range_start));

        map
    });

    // Debug
    maps.clone().enumerate().for_each(|(map_index, map)| {
        println!("Map {}:", map_index);
        map.iter().clone().for_each(|rule| println!("- {}", rule));
        println!("");
    });

    // Process the list of maps
    maps.into_iter().enumerate().for_each(|(map_index, map)| {
        // Debug
        println!("Seeds:");
        seeds.clone().iter().for_each(|seed| {
            println!("- {}", seed);
        });
        println!("Map #{}:", map_index);
        map.clone().iter().for_each(|rule| {
            println!("- {}", rule);
        });
        println!();

        // For each seed, apply the rules to according to the map.
        let mut seeds_iter = seeds.clone().into_iter();
        let mut maprules_iter = map.into_iter();

        let mut current_seed = seeds_iter.next();
        let mut current_maprule = maprules_iter.next();

        // Push the results of the rules into a new list
        let mut new_seeds: Vec<SeedRange> = Vec::new();

        while let Some(seed) = current_seed {
            if let Some(maprule) = current_maprule {
                if seed.start_index < maprule.source_range_start {
                    // seed: start: 0, length: 100
                    // maprule: source_start: 50, dest_start: 100, length: 300
                    let difference = maprule.source_range_start - seed.start_index;
                    new_seeds.push(SeedRange {
                        start_index: seed.start_index,
                        length: difference,
                    });

                    current_seed = Some(SeedRange {
                        start_index: seed.start_index + difference,
                        length: seed.length - difference,
                    })
                    // seed: start: 50, length: 50
                    // maprule: source_start: 50, dest_start: 100, length: 300
                } else if seed.start_index > maprule.source_range_start {
                    let difference = seed.start_index - maprule.source_range_start;

                    if difference <= maprule.range_length {
                        // seed: start: 50, length: 100
                        // maprule: source_start: 0, dest_start: 50, length: 300
                        current_maprule = Some(MapRule {
                            source_range_start: maprule.source_range_start + difference,
                            destination_range_start: maprule.destination_range_start + difference,
                            range_length: maprule.range_length - difference,
                        });
                        // maprule: source_start: 50, dest_start: 100, length: 250
                    } else {
                        // seed: start: 50, length: 100
                        // maprule: source_start: 0, dest_start: 50, length: 25
                        current_maprule = maprules_iter.next()
                    }
                } else {
                    if seed.length > maprule.range_length {
                        // seed: start: 0, length: 300
                        // maprule: source_start: 0, dest_start: 50, length: 100
                        let difference = seed.length - maprule.range_length;
                        let offset = maprule.destination_range_start - maprule.source_range_start;

                        new_seeds.push(SeedRange {
                            start_index: seed.start_index + offset,
                            length: seed.length - difference,
                        });

                        current_seed = Some(SeedRange {
                            start_index: seed.start_index + maprule.range_length,
                            length: difference,
                        });

                        current_maprule = maprules_iter.next();
                        // seed: start: 100, length: 200
                        // maprule: next()
                    } else {
                        // seed: start: 0, length: 100
                        // maprule: source_start: 0, dest_start: 50, length: 300
                        let difference = maprule.range_length - seed.length;
                        let offset = maprule.destination_range_start - maprule.source_range_start;

                        new_seeds.push(SeedRange {
                            start_index: seed.start_index + offset,
                            length: seed.length,
                        });

                        current_seed = seeds_iter.next();

                        current_maprule = Some(MapRule {
                            source_range_start: maprule.source_range_start + seed.length,
                            destination_range_start: maprule.destination_range_start + seed.length,
                            range_length: difference,
                        });
                    }
                }
            } else {
                new_seeds.push(seed);
                current_seed = seeds_iter.next();
            }
        }

        // Replace original seeds with new_seeds
        new_seeds.sort_by(|a, b| a.start_index.cmp(&b.start_index));
        seeds = new_seeds;
    });

    // Return the lowest value seed
    seeds.iter().map(|seed| seed.start_index).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4"
            .to_string();

        assert_eq!(part1(&contents), 35)
    }

    #[test]
    fn part1_input2() {
        let file_path: &Path = Path::new("inputs/day5.txt");
        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part1(&contents), 265018614)
    }

    #[test]
    fn part2_input1() {
        let contents: String = "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4"
            .to_string();

        assert_eq!(part2(&contents), 46)
    }

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

    //     assert_eq!(part2(&contents), todo!("Setup test output 2 for part2"))
    // }
}
