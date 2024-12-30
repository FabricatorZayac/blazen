use core::arch::wasm32::unreachable;

use rand::{rngs::SmallRng, RngCore, SeedableRng as _};
use wasm4::{draw::DrawIndex, tracef};

use crate::{
    button::Button,
    gfx::{texture::TEXTURE_BUFFER, Render},
    message::{Message, MESSAGE_BUF},
    util::Duration, Entropy, FrameCounter, MouseSemaphore
};
use crate::card::{
    animations::{idle1, idle2, idle3, idle4},
    deck::Deck,
    state::CardState,
};
use crate::animator::{animation_state::AnimationState, transform::{Rotate, Translate}};

use super::{Scene, ScenePtr};

pub const DEMO: *mut Demo = TEXTURE_BUFFER.wrapping_add(1) as *mut Demo;

impl ScenePtr for *mut Demo {
    fn init(self) {
        unsafe {
            self.write(Demo::new());
            tracef!("Demo initialized");
        }
    }
}

pub struct Demo {
    deck: Deck,
    deck_button: Button,

    hand: HandState,
    start_frame: u32,

    target: f32,
    
}
impl Demo {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            deck_button: Button::new(
                [140, 65],
                "Deck",
                DrawIndex::First,
                DrawIndex::Fourth,
                Message::DeckClicked,
            ),
            hand: HandState::default(),
            start_frame: FrameCounter::get() + 20,

            target: 400.0,
        }
    }
    pub fn get_deck(&self) -> &Deck {
        &self.deck
    }
}
impl Scene for Demo {
    fn update(&mut self, m: &MouseSemaphore) {
        if FrameCounter::get() < self.start_frame {
            // tracef!(
            //     "Start frame: {}, Current: {}",
            //     self.start_frame,
            //     FrameCounter::get(),
            // );
            return;
        }
        let mut rng = SmallRng::from_seed(Entropy::get());
        if self.start_frame == FrameCounter::get() {
            // init
            tracef!("Shuffling deck");
            self.deck.shuffle(&mut rng);

            while self.hand.cards.len() != self.hand.size {
                if let Some(card) = self.deck.draw() {
                    let pos = self.hand.cards.len();
                    let origin = [20 + pos as i32 * self.hand.size as i32 * 130 / 60, 135];
                    self.hand.cards.push(CardState::new(
                        pos,
                        card,
                        origin,
                        Some(AnimationState::new(&[
                            Rotate::new(90.0, 0.0).into(),
                            Translate::new(
                                [160.0 - origin[0] as f64, 80.0 - origin[1] as f64],
                                [0.0, 0.0],
                            ).into(),
                        ],
                        Duration::from_frames(rng.next_u32() % 10 + 10),
                        match rng.next_u32() % 4 {
                            0 => Some(idle1),
                            1 => Some(idle2),
                            2 => Some(idle3),
                            3 => Some(idle4),
                            _ => unreachable(),
                        })),
                    )).unwrap();
                }
            }
        }
        
        self.deck_button.update(m);
        self.hand.cards
            .iter_mut()
            .rev()
            .for_each(|card| card.handle_input(m));

        if let Some(Message::CardClicked(hand_idx)) = unsafe { MESSAGE_BUF } {
            if let Some(pos) = self.hand.selected.iter().position(|&e| e == hand_idx) {
                self.hand.selected.remove(pos);
            } else {
                self.hand.selected.push(hand_idx).ok();
            }
            tracef!("Total selected: {:?}", self.hand.selected);
        }

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
    }
}
impl Render for Demo {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        fb.line([0, 110], [160, 110], DrawIndex::Second);
        // card back cover
        fb.rect([140, 65], [40, 30], DrawIndex::First, DrawIndex::Fourth);

        self.deck_button.render(fb);

        if self.start_frame > FrameCounter::get() {
            // tracef!("Generating... {}/{}", FrameCounter::get(), self.start_frame);
            fb.text(
                "Generating...",
                [50, 70],
                DrawIndex::Second,
                DrawIndex::Transparent,
            );
            return;
        }

        self.hand.cards
            .iter()
            .for_each(|card| card.render(fb));
    }
}

pub struct HandState {
    pub size: usize,
    pub cards: heapless::Vec<CardState, 10>,
    pub selected: heapless::Vec<usize, 5>,
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
