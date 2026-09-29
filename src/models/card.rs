use crate::models::card::card_error::CardError;

pub mod card_error;

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
            Card::Ten => 10,
            Card::Jack => 10,
            Card::Queen => 10,
            Card::King => 10,
            Card::Ace => 11,
        }
    }
}

impl TryFrom<u32> for Card {
    type Error = CardError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Card::Two),
            3 => Ok(Card::Three),
            4 => Ok(Card::Four),
            5 => Ok(Card::Five),
            6 => Ok(Card::Six),
            7 => Ok(Card::Seven),
            8 => Ok(Card::Eight),
            9 => Ok(Card::Nine),
            10 => Ok(Card::Ten), // Jack, Queen and King are lost in translation
            11 => Ok(Card::Ace),
            _ => Err(value.into()),
        }
    }
}

impl From<Card> for usize {
    fn from(value: Card) -> Self {
        (u32::from(value) - 2) as usize
    }
}

impl TryFrom<usize> for Card {
    type Error = CardError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Card::try_from((value + 2) as u32).map_err(|err| {
            if let CardError::OutOfRangeU32(_) = err {
                value.into()
            } else {
                err
            }
        })
    }
}
