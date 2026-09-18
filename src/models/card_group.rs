use serde::Deserialize;
use serde::Serialize;

use crate::models::card::Card;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CardGroup {
    pub cards: [u32; 10],
}

impl CardGroup {
    pub fn add_card(&mut self, card_index: usize) {
        if card_index < 10 {
            self.cards[card_index] += 1;
        }
    }

    pub fn remove_card(&mut self, card_index: usize) {
        if card_index < 10 && self.cards[card_index] > 0 {
            self.cards[card_index] -= 1;
        }
    }

    pub fn hand_value(&self) -> u32 {
        let mut score = 0;
        for (i, card_amount) in self.cards.iter().enumerate() {
            score += ((i as u32) + 2) * card_amount
        }

        for _ in 0..self.cards[9] {
            if score <= 21 {
                break;
            }
            score -= 10;
        }
        
        score
    }
}

pub trait AddCard<T> {
    fn add_card(&mut self, card: T);
}

impl AddCard<Card> for CardGroup {
    fn add_card(&mut self, card: Card) {
        self.cards[usize::from(card)] += 1;
    }
}

impl AddCard<u32> for CardGroup {
    fn add_card(&mut self, card: u32) {
        self.cards[usize::from(Card::from(card))] += 1;
    }
}

impl AddCard<usize> for CardGroup {
    fn add_card(&mut self, card: usize) {
        self.cards[card] += 1;
    }
}

impl From<Vec<Card>> for CardGroup {
    fn from(value: Vec<Card>) -> Self {
        let mut cards = [0u32; 10];
        for card in value {
            cards[card as usize] += 1;
        }
        CardGroup { cards }
    }
}
