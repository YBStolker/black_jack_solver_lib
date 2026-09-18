use serde::Deserialize;
use serde::Serialize;

use crate::models::card_group::CardGroup;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GameState {
    pub player_hand: CardGroup,
    pub dealer_hand: CardGroup,
    pub seen_cards: CardGroup,
    pub shoe_size: u32,
}

impl GameState {
    pub fn new(
        player_hand: CardGroup,
        dealer_hand: CardGroup,
        seen_cards: CardGroup,
        shoe_size: u32,
    ) -> Self {
        GameState {
            player_hand,
            dealer_hand,
            seen_cards,
            shoe_size,
        }
    }

    pub fn get_result() -> Option<GameResult> {
        Some(GameResult::Win)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GameResult {
    Win,
    Loss,
    Tie,
}
