use hand_state::HandState;
use jokers::Jokers;
use rand::{rngs::SmallRng, SeedableRng};
use wasm4::{draw::DrawIndex, format::format_no_std, tracef};

use crate::{
    button::Button, card::deck::Deck, Entropy, MouseCompound, FORMAT_BUF
};
use crate::message::{InputHandler, Message, MessageHandler, Reader, Writer};
use crate::gfx::{texture::TEXTURE_BUFFER, Render};
use super::{Scene, ScenePtr};

mod hand_state;
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

    target: f32,
    score: f32,
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

            jokers: Default::default(),

            hand: Default::default(),
            play_button: Button::new(
                [125, 93],
                "Play",
                DrawIndex::Third,
                DrawIndex::Second,
                Message::PlayHand,
            ),
            discard_button: Button::new(
                [125, 82],
                "Dscd",
                DrawIndex::Second,
                DrawIndex::Third,
                Message::DiscardHand,
            ),

            target: 400.0,
            score: 0.0,
        }
    }
    pub fn get_deck(&self) -> &Deck {
        &self.deck
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
            },
            DemoState::Idle => { },
            DemoState::HandInProgress => { }, 
            DemoState::HandEnd => { },
        }
        self.hand.update(&self.state);
        self.jokers.update();
    }
}
impl InputHandler for Demo {
    fn handle_input(&self, mouse: &MouseCompound, tx: &mut Writer) {
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
                },
                Some(Message::DiscardHand) => self.hand.fill(&mut self.deck, &mut self.rng),
                _ => (),
            }
        }
    }
}
impl Render for Demo {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        fb.line([0, 115], [160, 115], DrawIndex::Second);
        // card back cover
        // fb.rect([140, 65], [40, 30], DrawIndex::First, DrawIndex::Fourth);

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

        fb.line([0, 160 - 115 + 22], [160, 160 - 115 + 22], DrawIndex::Second);

        self.deck_button.render(fb);
        self.play_button.render(fb);
        self.discard_button.render(fb);

        self.hand.render(fb);
        self.jokers.render(fb);
    }
}
