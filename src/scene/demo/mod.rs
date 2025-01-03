use crate::{animator::{animation_state::AnimationState, transform::{Rotate, Translate}}, card::{animations::random_idle, state::CardData}, util::Duration, CardState};
use jokers::Jokers;
use rand::{rngs::SmallRng, RngCore as _, SeedableRng};
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

enum DemoState {
    Init,
    Idle,
    HandInProgress,
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
        match self.state {
            DemoState::Init => {
                tracef!("Shuffling deck");
                self.rng = SmallRng::from_seed(Entropy::get());
                self.deck.shuffle(&mut self.rng);
                self.hand.fill(&mut self.deck, &mut self.rng);
                self.state = DemoState::Idle;
                self.rem_discards = self.max_discards;
                self.rem_hands = self.max_hands;
            },
            DemoState::Idle => { },
            DemoState::HandInProgress => { }, 
            DemoState::HandEnd => { },
        }
        // Selection hold
        for i in self.hand.selected.iter() {
            self.hand.cards[*i].set_animation(AnimationState::new(
                &[Translate::new([0.0, -10.0], [0.0, 0.0]).into()],
                Duration::from_secs(0.1),
                None,
            ));
        }
        self.hand.cards
            .iter_mut()
            .for_each(CardState::update);

        self.jokers.update();
    }
}
impl InputHandler for Demo {
    fn handle_input(&self, mouse: &MouseCompound, tx: &mut Writer) {
        if let DemoState::HandInProgress = self.state { return };

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
                    self.state = DemoState::HandInProgress;
                    self.rem_hands -= 1;

                    self.hand.selected = heapless::Vec::new();
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
            Some(Message::CardClicked(hand_idx)) if hand_idx < 100 => {
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
