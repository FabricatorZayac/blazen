use strum::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum Rank {
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

impl Rank {
    pub fn value(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Enhancement {
    Point,
    Mult,
    Stone,
}

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    enhancement: Option<Enhancement>,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
            enhancement: None,
        }
    }
    pub fn enhance(&mut self, enhancement: Enhancement) {
        self.enhancement = Some(enhancement);
    }
}
