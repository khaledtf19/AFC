use self::HandType::*;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};
const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum HandType {
    FIVE,
    FOUR,
    FULL,
    THREE,
    TWO,
    ONE,
    HIGH,
}
impl HandType {
    pub fn iter() -> impl Iterator<Item = HandType> {
        [FIVE, FOUR, FULL, THREE, TWO, ONE, HIGH].iter().copied()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Hand {
    cards: String,
    h_type: HandType,
    bid: u32,
    values: Vec<u32>,
}
impl Hand {
    pub fn new(cards: String, bid: u32) -> Self {
        let mut values: Vec<u32> = vec![];
        let mut hm: HashMap<char, u32> = HashMap::new();
        let mut kind = HandType::HIGH;
        for card in cards.chars() {
            let p = CARDS.iter().position(|ch| *ch == card).unwrap();
            values.push(p as u32 + 1);
            if let Some(value) = hm.get(&card).as_mut() {
                hm.insert(card, *value + 1);
            } else {
                hm.insert(card, 1);
            }
        }
        let hm_values = hm.values().collect::<Vec<&u32>>();
        if hm_values.len() == 1 {
            kind = HandType::FIVE;
        } else if hm_values.contains(&&4) {
            kind = HandType::FOUR;
        } else if hm_values.contains(&&3) {
            if hm_values.contains(&&2) {
                kind = HandType::FULL
            } else {
                kind = HandType::THREE
            }
        } else if hm_values.contains(&&2) {
            let filter = hm_values.iter().filter(|v| *v == &&2);
            if filter.count() == 2 {
                kind = HandType::TWO
            } else {
                kind = HandType::ONE
            }
        }
        Hand {
            cards,
            h_type: kind,
            bid,
            values,
        }
    }
    pub fn compare(&self, other_hand: &Hand) -> bool {
        for (i, num) in self.values.iter().enumerate() {
            if *num > other_hand.values[i] {
                return true;
            } else if *num < other_hand.values[i] {
                return false;
            }
        }
        false
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = Vec::new();

    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => resolve_line(line, &mut hands),
            Err(_) => {}
        }
    }

    hands.sort_by(|a, b| {
        let a_p = HandType::iter().position(|ht| ht == a.h_type).unwrap();
        let b_p = HandType::iter().position(|ht| ht == b.h_type).unwrap();

        match b_p.cmp(&a_p) {
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                if b.compare(a) {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            }
        }
    });
    let res = hands
        .iter()
        .enumerate()
        .fold(0, |state, (i, hand)| (hand.bid * (i as u32 + 1)) + state);

    println!("{:?}", res);
    Ok(())
}
fn resolve_line(line: String, hands: &mut Vec<Hand>) {
    let split: Vec<&str> = line.split(" ").collect();
    let hand = Hand::new(split[0].to_string(), split[1].parse().unwrap());
    hands.push(hand);
}
