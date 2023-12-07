advent_of_code::solution!(7);

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Card {
    Joker,
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from(b: u8) -> Card {
        match b {
            0x31..=0x39 => Card::Number(b - 0x30),
            0x54 => Card::Ten,
            0x4a => Card::Jack,
            0x51 => Card::Queen,
            0x4b => Card::King,
            0x41 => Card::Ace,
            _ => {
                panic!("\"{}\" is not a card", b as char);
            }
        }
    }

    fn from_value(b: u8) -> Card {
        match b {
            2..=9 => Card::Number(b),
            10 => Card::Ten,
            11 => Card::Jack,
            12 => Card::Queen,
            13 => Card::King,
            14 => Card::Ace,
            _ => {
                panic!("\"{}\" is not a card value", b);
            }
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            Card::Joker => 1,
            Card::Number(n) => *n,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_u8().cmp(&other.as_u8())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq)]
enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Clone, Debug)]
pub struct Hand {
    hand_type: HandType,
    original_cards: Vec<Card>,
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(text: &str) -> Hand {
        let mut text = text.split_whitespace();
        let mut cards: Vec<Card> = text.next()
            .unwrap()
            .chars()
            .map(|card| Card::from(card as u8))
            .collect();
        let original_cards: Vec<Card> = cards.to_vec();
        cards.sort_unstable_by(|a, b| b.cmp(a));

        let bid = text.next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut counts = [0u8; 15];
        for c in cards.drain(..) {
            counts[c.as_u8() as usize] += 1;
        }

        let mut five: Option<Card> = None;
        let mut fours: Option<Card> = None;
        let mut three: Option<Card> = None;
        let mut high_two: Option<Card> = None;
        let mut low_two: Option<Card> = None;
        let mut rest: Vec<Card> = Vec::with_capacity(5);

        for (card_n, count_n) in counts.iter().enumerate().rev() {
            match count_n {
                1 => { rest.push(Card::from_value(card_n as u8)); }
                2 => {
                    match high_two {
                        None => { high_two = Some(Card::from_value(card_n as u8)); },
                        Some(_) => { low_two = Some(Card::from_value(card_n as u8)); },
                    }
                }
                3 => { three = Some(Card::from_value(card_n as u8)); }
                4 => { fours = Some(Card::from_value(card_n as u8)); }
                5 => { five = Some(Card::from_value(card_n as u8)); }
                _ => {}
            }
        }
        
        if let Some(c) = five {
            for _ in 0..5 { cards.push(c); }
            return Hand { hand_type: HandType::Five, original_cards, cards, bid };
        }

        if let Some(c) = fours {
            for _ in 0..4 { cards.push(c); }
            cards.append(&mut rest);
            return Hand { hand_type: HandType::Four, original_cards, cards, bid };
        }

        if let Some(c) = three {
            let mut hand_type: HandType = HandType::Three;
            for _ in 0..3 { cards.push(c); }

            if let Some(c) = high_two {
                hand_type = HandType::FullHouse;
                for _ in 0..2 { cards.push(c); }
            } else {
                cards.append(&mut rest);
            }

            return Hand { hand_type, original_cards, cards, bid };
        }

        if let Some(c) = high_two {
            let mut hand_type: HandType = HandType::Pair;
            for _ in 0..2 { cards.push(c); }

            if let Some(c) = low_two {
                hand_type = HandType::TwoPair;
                for _ in 0..2 { cards.push(c); }
            }
            cards.append(&mut rest);

            return Hand { hand_type, original_cards, cards, bid };
        }

        Hand { hand_type: HandType::High, original_cards, cards: rest, bid }
    }

    fn jacks_into_jokers(&mut self) {
        self.cards.iter_mut()
            .for_each(|card| match card {
                Card::Jack => { *card = Card::Joker; },
                _ => {}
            });
        self.original_cards.iter_mut()
            .for_each(|card| match card {
                Card::Jack => { *card = Card::Joker; },
                _ => {}
            });
    }

    fn use_jokers(&mut self) {
        let jokers: usize = self.original_cards.iter().filter(|&card| *card == Card::Joker).count();
        if jokers == 0 { return; }
        match self.hand_type {
            HandType::Five => { self.hand_type = HandType::Five },
            HandType::Four => { self.hand_type = HandType::Five },
            HandType::FullHouse => { self.hand_type = HandType::Five },
            HandType::Three => { self.hand_type = HandType::Four },
            HandType::TwoPair => {
                match jokers {
                    1 => { self.hand_type = HandType::FullHouse },
                    2 => { self.hand_type = HandType::Four },
                    _ => {}
                }
            }
            HandType::Pair => { self.hand_type = HandType::Three },
            HandType::High => { self.hand_type = HandType::Pair },
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if &self.hand_type == &other.hand_type {
            &self.original_cards == &other.original_cards
        } else {
            false
        }
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.partial_cmp(&other.hand_type).unwrap() {
            Ordering::Equal => self.original_cards.cmp(&other.original_cards),
            x => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines()
        .map(|line| Hand::new(line))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = parse(input);
    hands.sort_unstable_by(|a, b| a.cmp(b));
    hands.into_iter()
        .enumerate()
        .fold(Some(0), |acc, (i, hand)| {
            Some(acc.unwrap() + ((i as u32 + 1) * hand.bid))
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = parse(input).into_iter()
        .map(|mut hand| {
            hand.jacks_into_jokers();
            hand.use_jokers();
            hand
        })
        .collect();
    hands.sort_unstable_by(|a, b| a.cmp(b));
    hands.into_iter()
        .enumerate()
        .fold(Some(0), |acc, (i, hand)| {
            Some(acc.unwrap() + ((i as u32 + 1) * hand.bid))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
