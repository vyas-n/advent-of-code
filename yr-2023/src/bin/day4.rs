use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() {
    // let contents: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    //     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    //     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    //     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    //     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    //     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    //     .to_string();

    let file_path: &Path = Path::new("inputs/day4.txt");

    let contents = fs::read_to_string(&file_path).expect(
        format!(
            "Should have been able to read the file at file path: {:?}",
            &file_path
        )
        .as_str(),
    );

    println!("Answer: {}", part2(&contents))
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

#[derive(Debug)]
struct CardParseError;

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This section trims: "Card #:"
        Ok(s.trim()
            .strip_prefix("Card")
            .and_then(|input| {
                Some(
                    input
                        .trim_start()
                        .trim_start_matches(char::is_numeric)
                        .trim_start_matches(":")
                        .trim_start(),
                )
            })
            // Splits the string into 2 sections
            .and_then(|input| input.split_once('|'))
            // Convert the 2 strings into lists of numbers wrapped into a Card
            .and_then(|(winning_numbers, numbers_you_have)| {
                Some(Card {
                    winning_numbers: winning_numbers
                        .trim()
                        .split_ascii_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect(),
                    numbers_you_have: numbers_you_have
                        .trim()
                        .split_ascii_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect(),
                })
            })
            // If any of the chained operations fail, then return CardParseError
            .ok_or(CardParseError)?)
    }
}

fn part1(input: &str) -> u32 {
    // Parse Cards
    let cards = input
        .trim()
        .lines()
        .map(|line| Card::from_str(line.trim()).unwrap());

    // Debug
    cards.clone().for_each(|card| println!("{:?}", card));

    // Calculate value of each card
    cards
        .map(|card| {
            let count = card
                .numbers_you_have
                .into_iter()
                // Filter out the numbers_you_have to only winning numbers
                .filter(|number| card.winning_numbers.contains(number))
                // Count the remaining numbers_you_have
                .count();

            // Convert count to points
            if count == 0 {
                0
            } else {
                2u32.pow(count as u32 - 1)
            }
        })
        // Sum the points from each card
        .sum()
}

fn part2(input: &str) -> u32 {
    // Parse Cards
    let cards = input
        .trim()
        .lines()
        .map(|line| Card::from_str(line.trim()).unwrap());

    // Initialize a list with an initial value of 1 for each card
    let mut num_cards = (0..cards.clone().collect::<Vec<Card>>().len())
        .map(|_| 1)
        .collect::<Vec<u32>>();

    // Debug
    cards
        .clone()
        .enumerate()
        .for_each(|(index, card)| println!("{}: {:?}", index, card));

    // Process the cards
    cards.enumerate().for_each(|(index, card)| {
        let count = card
            .numbers_you_have
            .into_iter()
            // Filter out the numbers_you_have to only winning numbers
            .filter(|number| card.winning_numbers.contains(number))
            // Count the remaining numbers_you_have
            .count();

        (index + 1..(index + 1 + count))
            .for_each(|card_index| num_cards[card_index] += num_cards[index]);
    });

    // Return the sum of the num_cards
    num_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        assert_eq!(part1(&contents), 13)
    }

    #[test]
    fn part1_input2() {
        let file_path: &Path = Path::new("inputs/day4.txt");
        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part1(&contents), 23673)
    }

    #[test]
    fn part2_input1() {
        let contents: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        assert_eq!(part2(&contents), 30)
    }

    #[test]
    fn part2_input2() {
        let file_path: &Path = Path::new("inputs/day4.txt");

        let contents = fs::read_to_string(&file_path).expect(
            format!(
                "Should have been able to read the file at file path: {:?}",
                &file_path
            )
            .as_str(),
        );

        assert_eq!(part2(&contents), 12263631)
    }
}
