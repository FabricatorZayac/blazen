use crate::{card::{animations::random_idle, state::CardData, Rank}, util::Duration, CardState};
use crate::animator::{animation_state::AnimationState, transform::{Rotate, Translate}};
use enumflags2::BitFlags;
use jokers::Jokers;
use rand::{rngs::SmallRng, RngCore as _, SeedableRng};
use strum::{EnumIter, IntoEnumIterator};
use wasm4::{draw::DrawIndex, format::format_no_std, tracef};

use crate::{
    button::Button, card::deck::Deck, Entropy, MouseCompound, FORMAT_BUF
};
use crate::message::{InputHandler, Message, MessageHandler, Reader, Writer};
use crate::gfx::{texture::TEXTURE_BUFFER, Render};
use super::{Scene, ScenePtr};

mod jokers;

pub const DEMO: *mut Demo = TEXTURE_BUFFER.wrapping_add(1) as *mut Demo;

impl ScenePtr for *mut Demo {
    fn init(self) {
        unsafe {
            self.write(Demo::new());
            tracef!("Demo initialized");
        }
    }
    fn get(self) -> &'static mut dyn Scene {
        unsafe {self.as_mut()}.unwrap()
    }
}

// NOTE: Should replace with a bitflag for "Contains" type jokers
#[repr(u16)]
#[enumflags2::bitflags]
#[derive(EnumIter, Copy, Clone)]
enum PokerHand {
    HighCard,
    Pair,
    TwoPair,
    Three,
    Straight,
    Flush,
    FullHouse,
    Four,
    // StraightFlush,

    Five,
    // FlushHouse,
    // FlushFive,
}
impl PokerHand {
    fn min_cards(&self) -> usize {
        match self {
            PokerHand::HighCard => 1,
            PokerHand::Pair => 2,
            PokerHand::Three => 3,
            PokerHand::TwoPair | PokerHand::Four => 4,
            PokerHand::Straight
            | PokerHand::Flush
            | PokerHand::FullHouse
            // | PokerHand::StraightFlush
            | PokerHand::Five
            // | PokerHand::FlushHouse
            // | PokerHand::FlushFive
            => 5,
        }
    }
}
struct Score {
    points: f32,
    mult: f32,
}
impl From<(f32, f32)> for Score {
    fn from(value: (f32, f32)) -> Self {
        Self { points: value.0, mult: value.1 }
    }
}
impl From<BitFlags<PokerHand>> for Score {
    fn from(value: BitFlags<PokerHand>) -> Self {
        if value.contains(PokerHand::Flush) {
            if value.contains(PokerHand::Straight) {
                // Straight Flush
                return (100.0, 8.0).into();
            }
            if value.contains(PokerHand::FullHouse) {
                // Flush House
                return (140.0, 14.0).into();
            }
            if value.contains(PokerHand::Five) {
                // Flush Five
                return (160.0, 16.0).into();
            }
        }
        for hand in PokerHand::iter().rev() {
            if value.contains(hand) {
                return hand.into();
            }
        }
        unreachable!()
    }
}
impl From<PokerHand> for Score {
    fn from(value: PokerHand) -> Self {
        match value {
            PokerHand::HighCard => (5.0, 1.0),
            PokerHand::Pair => (10.0, 2.0),
            PokerHand::TwoPair => (20.0, 2.0),
            PokerHand::Three => (30.0, 3.0),
            PokerHand::Straight => (30.0, 4.0),
            PokerHand::Flush => (35.0, 4.0),
            PokerHand::FullHouse => (40.0, 4.0),
            PokerHand::Four => (60.0, 7.0),
            // PokerHand::StraightFlush => (100.0, 8.0),
            PokerHand::Five => (120.0, 12.0),
            // PokerHand::FlushHouse => (140.0, 14.0),
            // PokerHand::FlushFive => (160.0, 16.0),
        }.into()
    }
}
enum DemoState {
    Init,
    Idle,
    InitPlay,
    Play(Score),
    HandEnd,
}
pub struct Demo {
    state: DemoState,
    rng: SmallRng,

    deck: Deck,
    deck_button: Button,

    jokers: Jokers,
    hand: HandState,

    play_button: Button,
    discard_button: Button,

    target: u32,
    score: u32,

    max_hands: u8,
    rem_hands: u8,

    max_discards: u8,
    rem_discards: u8,
}
impl Demo {
    pub fn new() -> Self {
        Self {
            state: DemoState::Init,
            rng: SmallRng::from_seed(Entropy::get()),

            deck: Deck::new(),
            deck_button: Button::new(
                [125, 104],
                "Deck",
                DrawIndex::First,
                DrawIndex::Fourth,
                Message::DeckClicked,
            ),
            play_button: Button::new(
                [125, 68],
                "Play",
                DrawIndex::Third,
                DrawIndex::Second,
                Message::PlayHand,
            ),
            discard_button: Button::new(
                [0, 68],
                "Discard",
                DrawIndex::Second,
                DrawIndex::Third,
                Message::DiscardHand,
            ),

            target: 400,
            score: 0,

            jokers: Default::default(),
            hand: Default::default(),

            max_hands: 4,
            max_discards: 3,
            rem_hands: Default::default(),
            rem_discards: Default::default(),
        }
    }
}
impl Scene for Demo {
    fn update(&mut self) {
        match &mut self.state {
            DemoState::Init => {
                tracef!("Shuffling deck");
                self.rng = SmallRng::from_seed(Entropy::get());
                self.deck.shuffle(&mut self.rng);
                self.hand.fill(&mut self.deck, &mut self.rng);
                self.state = DemoState::Idle;
                self.rem_discards = self.max_discards;
                self.rem_hands = self.max_hands;
            },
            DemoState::Idle => {
                // Selection hold
                self.hand.selected.iter().for_each(|i| {
                    self.hand.cards[*i].set_animation(AnimationState::new(
                        &[Translate::new([0.0, -10.0], [0.0, 0.0]).into()],
                        Duration::from_secs(0.1),
                        None,
                    ));
                });
            },
            DemoState::InitPlay => { 
                self.state = DemoState::Play(self.hand.match_poker().into());
                tracef!("Initialized Play");
            },
            DemoState::Play(score) => {
                // for i in self.hand.selected.iter() {
                //     let CardData::Playing(card) = self.hand.cards[*i].card() else { unreachable!() };
                // }
            }, 
            DemoState::HandEnd => { },
        }
        self.hand.cards
            .iter_mut()
            .for_each(CardState::update);

        self.jokers.update();
    }
}
impl InputHandler for Demo {
    fn handle_input(&self, mouse: &MouseCompound, tx: &mut Writer) {
        if let DemoState::Play(_) = self.state { return };

        self.hand.handle_input(mouse, tx);
        self.jokers.handle_input(mouse, tx);

        self.deck_button.handle_input(mouse, tx);
        self.play_button.handle_input(mouse, tx);
        self.discard_button.handle_input(mouse, tx);
    }
}
impl MessageHandler for Demo {
    fn handle_message(&mut self, rx: &Reader) {
        self.hand.handle_message(rx);
        self.jokers.handle_message(rx);
        if let DemoState::Idle = self.state {
            match rx.read() {
                Some(Message::PlayHand) => {
                    self.state = DemoState::InitPlay;
                    self.rem_hands -= 1;
                },
                Some(Message::DiscardHand) if self.rem_discards > 0 && self.hand.selected.len() > 0 => {
                    self.rem_discards -= 1;
                    self.hand.cards.retain(|card| !self.hand.selected.contains(&card.id()));
                    self.hand.selected = heapless::Vec::new();
                    self.hand.fill(&mut self.deck, &mut self.rng);
                },
                _ => (),
            }
        }
    }
}
impl Render for Demo {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        fb.line([0, 115], [160, 115], DrawIndex::Second);
        fb.line([0, 67], [160, 67], DrawIndex::Second);

        fb.rect([0, 0], [160, 11], DrawIndex::Third, DrawIndex::Second);
        fb.text(format_no_std::show(
            unsafe { FORMAT_BUF.assume_init_mut() },
            format_args!("Score: {}", self.score),
        ).unwrap(), [2, 2], DrawIndex::Fourth, DrawIndex::Transparent);

        fb.rect([0, 11], [160, 11], DrawIndex::Second, DrawIndex::Third);
        fb.text(format_no_std::show(
            unsafe { FORMAT_BUF.assume_init_mut() },
            format_args!("Target: {}", self.target),
        ).unwrap(), [2, 13], DrawIndex::Fourth, DrawIndex::Transparent);

        self.play_button.render(fb);
        fb.rect([125, 78], [35, 11], DrawIndex::Third, DrawIndex::Second);
        fb.text(format_no_std::show(
            unsafe { FORMAT_BUF.assume_init_mut() },
            format_args!("{}/{}", self.rem_hands, self.max_hands),
        ).unwrap(), [127, 80], DrawIndex::Fourth, DrawIndex::Transparent);

        self.discard_button.render(fb);
        fb.rect([0, 78], [59, 11], DrawIndex::Second, DrawIndex::Third);
        fb.text(format_no_std::show(
            unsafe { FORMAT_BUF.assume_init_mut() },
            format_args!("{}/{}", self.rem_discards, self.max_discards),
        ).unwrap(), [2, 80], DrawIndex::Fourth, DrawIndex::Transparent);
        
        self.deck_button.render(fb);

        self.hand.cards
            .iter()
            .for_each(|card| card.render(fb));
        self.jokers.render(fb);
    }
}

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
            Some(Message::CardClicked(hand_idx)) if hand_idx < 0xFF => {
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
                                (old_origin[0] - card.origin()[0]) as f32,
                                (old_origin[1] - card.origin()[1]) as f32,
                            ],
                            [0.0, 0.0]).into()],
                            Duration::from_frames(i as u32 * 5),
                            None,
                        ));
                    });
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
    pub fn fill(&mut self, deck: &mut Deck, animation_rng: &mut SmallRng) {
        self.cards
            .iter_mut()
            .enumerate()
            .for_each(|(i, card)| {
                let old_origin = card.origin();
                card.set_origin([20 + i as i32 * self.size as i32 * 130 / 60, 140]);
                card.set_animation(AnimationState::new(
                    &[Translate::new([
                         (old_origin[0] - card.origin()[0]) as f32,
                         (old_origin[1] - card.origin()[1]) as f32,
                    ], [0.0, 0.0]).into()],
                    Duration::from_secs(0.2),
                    Some(random_idle(animation_rng)),
                ));
                card.set_id(i);
            }); 
        while self.cards.len() != self.size {
            match deck.draw() {
                Some(card) => {
                    let pos = self.cards.len();
                    let origin = [20 + pos as i32 * self.size as i32 * 130 / 60, 140];
                    self.cards.push(CardState::new(
                        pos,
                        CardData::Playing(card),
                        origin,
                        Some(AnimationState::new(
                            &[
                                Translate::new(
                                    [160.0 - origin[0] as f32, 80.0 - origin[1] as f32],
                                    [0.0, 0.0],
                                ).into(),
                                Rotate::new(90.0, 0.0).into(),
                            ],
                            Duration::from_frames(animation_rng.next_u32() % 10 + 10),
                            Some(random_idle(animation_rng)),
                        )),
                    )).unwrap();
                }
                None => break,
            }
        }
        // self.cards.sort_unstable_by(|left, right| {
        //     let (CardData::Playing(left), CardData::Playing(right)) = (left.card(), right.card()) else { unreachable!() };
        //     left.cmp(right)
        // });
    }
    fn match_poker(&self) -> BitFlags<PokerHand> {
        let mut played_cards = self.selected
            .iter()
            .map(|idx| {
                let CardData::Playing(card) = self.cards[*idx].card() else {unreachable!()};
                card
            })
            .collect::<heapless::Vec<_, 5>>();
        played_cards.sort_unstable_by(|left, right| left.rank().cmp(&right.rank()));
        tracef!("played: {:?}", played_cards);

        // NOTE: This is really shit code, I should rewrite it when my mind is not a haze
        let mut current_rank: Rank = played_cards[0].rank();
        let mut sets = [0; 2];
        let mut idx = 0;
        for card in &played_cards {
            if card.rank() == current_rank {
                sets[idx] += 1;
            } else if idx != 1 && sets[idx] >= 2 {
                idx = 1;
                sets[idx] += 1;
            }
            current_rank = card.rank();
        }
        let sets = sets.iter().filter(|i| **i >= 2).collect::<heapless::Vec<_, 2>>();
        tracef!("Sets: {:?}", sets);

        let mut matched: BitFlags<PokerHand> = PokerHand::HighCard.into();
        matched |= match sets.as_slice() {
            [2] => PokerHand::Pair,
            [3] => PokerHand::Three,
            [4] => PokerHand::Four,
            [5] => PokerHand::Five,
            [2, 2] => PokerHand::TwoPair,
            [3, 2] => PokerHand::FullHouse,
            [] => PokerHand::HighCard,
            _ => unreachable!(),
        };

        if played_cards.len() == 5 {
            // Flush check
            let mut flush = true;
            for i in &played_cards {
                if i.suit() != played_cards[0].suit() {
                    flush = false;
                }
            }
            if flush {
                matched |= PokerHand::Flush;
            }

            // Straight check
            let mut straight = true;
            for (i, card) in played_cards[1..].iter().enumerate() {
                if card.rank() as u8 != played_cards[i-1].rank() as u8 - 1 {
                    // TODO: Ace, 2... straight detect
                    straight = false;
                }
            }
            if straight {
                matched |= PokerHand::Straight;
            }
        }

        matched
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
