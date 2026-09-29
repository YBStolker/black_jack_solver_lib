use crate::models::card_group::CardGroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardGroupT1(u32);

impl CardGroup for CardGroupT1 {
    fn hand_score(&self) -> u32 {
        self.0
    }

    fn is_soft(&self) -> bool {
        false
    }

    fn is_blackjack(&self) -> bool {
        false
    }

    fn is_blackjack_possible(&self) -> bool {
        false
    }
}
