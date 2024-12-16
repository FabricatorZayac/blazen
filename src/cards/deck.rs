use super::card::{Card, Rank, Suit};
use heapless::Vec;
use strum::IntoEnumIterator;

pub enum DeckType {
    Default,
    // TODO: potentially add deck effects maybe
}

pub struct Deck {
    cards: Vec<Card, 255>,
    kind: DeckType,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card, 255> = Vec::new();
        
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                match cards.push(Card::new(suit, rank)) {
                    Ok(_) => {},
                    Err(_) => unreachable!(), // Can't overflow. 52/255
                }
            }
        }

        Self {
            cards,
            kind: DeckType::Default,
        }
    }
}
