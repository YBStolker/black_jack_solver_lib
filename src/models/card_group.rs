pub mod card_group_t1;

pub trait CardGroup {
    fn hand_score(&self) -> u32;
    fn is_soft(&self) -> bool;
    fn is_blackjack(&self) -> bool;
    fn is_blackjack_possible(&self) -> bool;

    fn is_bust(&self) -> bool {
        self.hand_score() > 21
    }
}
