use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hello")
    }
}

impl From<Card> for u32 {
    fn from(value: Card) -> Self {
        match value {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten | Card::Jack | Card::Queen | Card::King => 10,
            Card::Ace => 11,
        }
    }
}

impl From<Card> for usize {
    fn from(value: Card) -> Self {
        ((value as u32) - 2) as usize
    }
}

impl From<u32> for Card {
    fn from(value: u32) -> Self {
        match value {
            2 => Card::Two,
            3 => Card::Three,
            4 => Card::Four,
            5 => Card::Five,
            6 => Card::Six,
            7 => Card::Seven,
            8 => Card::Eight,
            9 => Card::Nine,
            10 => Card::Ten, // 10 may also be a Jack, Queen or King, but that information is lost in translation.
            11 => Card::Ace,
            _ => panic!("value out of bounds for Card"),
        }
    }
}

impl From<usize> for Card {
    fn from(value: usize) -> Self {
        Card::from((value + 2) as u32)
    }
}
