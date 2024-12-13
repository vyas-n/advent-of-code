use std::{cmp::Ordering, path::Path};
use std::{fmt::Display, fs};

fn const_contents() -> String {
    let contents: &str = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    contents.to_string()
}

fn file_contents() -> String {
    let file_path: &Path = Path::new("inputs/day7.txt");

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

#[derive(Clone, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn value(&self) -> u64 {
        return match *self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
            Self::Joker => 1,
        };
    }
}

#[derive(Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|character| match character {
                Card::Ace => 'A',
                Card::King => 'K',
                Card::Queen => 'Q',
                Card::Jack => 'J',
                Card::Ten => 'T',
                Card::Nine => '9',
                Card::Eight => '8',
                Card::Seven => '7',
                Card::Six => '6',
                Card::Five => '5',
                Card::Four => '4',
                Card::Three => '3',
                Card::Two => '2',
                Card::Joker => 'J',
            })
            .fold(String::new(), |a, b| format!("{}{}", a, b));

        write!(
            f,
            "[{}] bid: {:3} ({:?} -> {:?})",
            cards,
            self.bid,
            self.handtype(),
            self.joker_handtype()
        )
    }
}

impl Hand {
    fn handtype(&self) -> HandType {
        let counts = [
            Card::Ace,
            Card::King,
            Card::Queen,
            Card::Jack,
            Card::Ten,
            Card::Nine,
            Card::Eight,
            Card::Seven,
            Card::Six,
            Card::Five,
            Card::Four,
            Card::Three,
            Card::Two,
        ]
        .iter()
        .filter(|card| self.cards.contains(card))
        .map(|card| (card, self.cards.iter().filter(|c| *c == card).count()));

        return if counts.clone().any(|(_, count)| count == 5) {
            HandType::FiveOfAKind
        } else if counts.clone().any(|(_, count)| count == 4) {
            HandType::FourOfAKind
        } else if counts.clone().any(|(_, count)| count == 3)
            && counts.clone().any(|(_, count)| count == 2)
        {
            HandType::FullHouse
        } else if counts.clone().any(|(_, count)| count == 3) {
            HandType::ThreeOfAKind
        } else if counts.clone().filter(|(_, count)| *count == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.clone().any(|(_, count)| count == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        };
    }

    fn joker_handtype(&self) -> HandType {
        if let Some((index, _)) = self
            .cards
            .iter()
            .enumerate()
            .find(|(_, card)| **card == Card::Joker)
        {
            [
                Card::Ace,
                Card::King,
                Card::Queen,
                Card::Jack,
                Card::Ten,
                Card::Nine,
                Card::Eight,
                Card::Seven,
                Card::Six,
                Card::Five,
                Card::Four,
                Card::Three,
                Card::Two,
            ]
            .into_iter()
            .map(|replacement_card| {
                let mut replacement_hand = self.clone();
                replacement_hand.cards[index] = replacement_card;

                replacement_hand.joker_handtype()
            })
            .fold(HandType::HighCard, |handtype_a, handtype_b| {
                if handtype_b.value() > handtype_a.value() {
                    handtype_b
                } else {
                    handtype_a
                }
            })
        } else {
            self.handtype()
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u64 {
        return match *self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        };
    }
}

fn part1(input: &str) -> u64 {
    // Input Parsing
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| line.trim().split_once(' ').unwrap())
        .map(|(cards, bid)| Hand {
            cards: cards
                .trim()
                .chars()
                .map(|character| match character {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("Yo this isn't a valid card!"),
                })
                .collect(),
            bid: bid.parse().unwrap(),
        })
        .collect();

    // Debug
    println!("Initial Hand:");
    hands.iter().for_each(|hand| {
        println!("- {}", hand);
    });
    println!();

    // Sort the hands in order of rank
    hands.sort_by(|hand_a, hand_b| {
        let handtype_cmp = hand_a.handtype().value().cmp(&hand_b.handtype().value());

        if handtype_cmp != Ordering::Equal {
            return handtype_cmp;
        }

        hand_a
            .cards
            .iter()
            .zip(hand_b.cards.iter())
            .map(|(card_a, card_b)| card_a.value().cmp(&card_b.value()))
            .fold(Ordering::Equal, |a, b| {
                return if a != Ordering::Equal { a } else { b };
            })
    });

    // Debug
    println!("Sorted Hand:");
    hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1, hand))
        .map(|(rank, hand)| {
            let hand_value = rank * hand.bid;

            println!("{:4}. {} = {}", rank, hand, hand_value);

            hand_value
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    // Input Parsing
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| line.trim().split_once(' ').unwrap())
        .map(|(cards, bid)| Hand {
            cards: cards
                .trim()
                .chars()
                .map(|character| match character {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Joker,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("Yo this isn't a valid card!"),
                })
                .collect(),
            bid: bid.parse().unwrap(),
        })
        .collect();

    // Debug
    println!("Initial Hand:");
    hands.iter().for_each(|hand| {
        println!("- {}", hand);
    });
    println!();

    // Sort the hands in order of rank
    hands.sort_by(|hand_a, hand_b| {
        let handtype_cmp = hand_a
            .joker_handtype()
            .value()
            .cmp(&hand_b.joker_handtype().value());

        if handtype_cmp != Ordering::Equal {
            return handtype_cmp;
        }

        hand_a
            .cards
            .iter()
            .zip(hand_b.cards.iter())
            .map(|(card_a, card_b)| card_a.value().cmp(&card_b.value()))
            .fold(Ordering::Equal, |a, b| {
                return if a != Ordering::Equal { a } else { b };
            })
    });

    // Debug
    println!("Sorted Hand:");
    hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1, hand))
        .map(|(rank, hand)| {
            let hand_value = rank * hand.bid;

            println!("{:4}. {} = {}", rank, hand, hand_value);

            hand_value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input1() {
        let contents: String = const_contents();

        assert_eq!(part1(&contents), 6440)
    }

    #[test]
    fn part1_input2() {
        let contents = file_contents();

        assert_eq!(part1(&contents), 252656917)
    }

    #[test]
    fn part2_input1() {
        let contents: String = const_contents();

        assert_eq!(part2(&contents), 5905)
    }

    #[test]
    fn part2_input2() {
        let contents = file_contents();

        assert_eq!(part2(&contents), 253499763)
    }
}
