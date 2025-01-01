use rand::{rngs::SmallRng, RngCore as _};
use wasm4::tracef;

use crate::{animator::{animation_state::AnimationState, transform::{Rotate, Translate}}, card::{animations::{idle1, idle2, idle3, idle4}, deck::Deck, state::{CardData, CardState}}, gfx::Render, message::{InputHandler, Message, MessageHandler, Reader}, util::{Duration, MouseCompound}};

use super::DemoState;

pub struct HandState {
    size: usize,
    cards: heapless::Vec<CardState, 10>,
    selected: heapless::Vec<usize, 5>,
}

impl MessageHandler for HandState {
    fn handle_message(&mut self, rx: &Reader) {
        self.cards
            .iter_mut()
            .for_each(|card| card.handle_message(rx));

        match rx.read() {
            Some(Message::CardClicked(hand_idx)) => {
                if let Some(pos) = self.selected.iter().position(|&e| e == hand_idx) {
                    self.selected.remove(pos);
                } else {
                    self.selected.push(hand_idx).ok();
                }
                tracef!("Total selected: {:?}", self.selected);
            },
            Some(Message::PlayHand) => {
                self.selected
                    .iter()
                    .enumerate()
                    .for_each(|(i, idx)| {
                        // tracef!("Setting animation for card pos: {}", idx);
                        let card = &mut self.cards[*idx];
                        let old_origin = card.origin();
                        card.set_origin([15 * 5 / self.selected.len() as i32 + i as i32 * 32 * 5 / self.selected.len() as i32, 90]);
                        card.set_animation(AnimationState::new(&[
                            Translate::new([
                                (old_origin[0] - card.origin()[0]) as f64,
                                (old_origin[1] - card.origin()[1]) as f64,
                            ],
                            [0.0, 0.0]).into()],
                            Duration::from_frames(i as u32 * 5),
                            None,
                        ));
                    });
            },
            Some(Message::DiscardHand) => {
                self.cards.retain(|card| !self.selected.contains(&card.id()));
                self.selected = heapless::Vec::new();
            },
            _ => (),
        }
    }
}

impl InputHandler for HandState {
    fn handle_input(&self, m: &MouseCompound, tx: &mut crate::message::Writer) {
        self.cards
            .iter()
            .rev()
            .for_each(|card| card.handle_input(m, tx));
    }
}

impl HandState {
    pub fn update(&mut self, state: &DemoState) {
        match state {
            DemoState::Idle => {
                for i in self.selected.iter() {
                    self.cards[*i].set_animation(AnimationState::new(
                        &[Translate::new([0.0, -10.0], [0.0, 0.0]).into()],
                        Duration::from_secs(0.1),
                        None,
                    ));
                }
            },
            _ => (),
        }

        self.cards
            .iter_mut()
            .for_each(CardState::update);
    }

    pub fn fill(&mut self, deck: &mut Deck, animation_rng: &mut SmallRng) {
        while self.cards.len() != self.size {
            if let Some(card) = deck.draw() {
                let pos = self.cards.len();
                let origin = [20 + pos as i32 * self.size as i32 * 130 / 60, 140];
                self.cards.push(CardState::new(
                    pos,
                    CardData::Playing(card),
                    origin,
                    Some(AnimationState::new(&[
                        Rotate::new(90.0, 0.0).into(),
                        Translate::new(
                            [160.0 - origin[0] as f64, 80.0 - origin[1] as f64],
                            [0.0, 0.0],
                        ).into(),
                    ],
                    Duration::from_frames(animation_rng.next_u32() % 10 + 10),
                    match animation_rng.next_u32() % 4 {
                        0 => Some(idle1),
                        1 => Some(idle2),
                        2 => Some(idle3),
                        3 => Some(idle4),
                        _ => unreachable!(),
                    })),
                )).unwrap();
            }
        }
    }
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            size: 8,
            cards: heapless::Vec::new(),
            selected: heapless::Vec::new(),
        }
    }
}

impl Render for HandState {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        self.cards
            .iter()
            .for_each(|card| card.render(fb));
    }
}

pub enum PokerHand {
    HighCard,
    Pair,
    TwoPair,
    Three,
    Straight,
    Flush,
    FullHouse,
    Four,

    FlushHouse,
    Five,
    FlushFive,
}

// impl From<&HandState> for PokerHand {
//     fn from(value: &HandState) -> Self {
//         let cards = value.selected
//             .iter()
//             .map(|index| &value.cards[*index])
//             .collect::<heapless::Vec<_, 5>>();
//
//         if cards.len() == 0 {
//             unreachable!()
//         }
//
//         if cards.len() == 1 {
//             return Self::HighCard;
//         }
//
//         // let same_rank = cards.iter().filter()
//     }
// }
