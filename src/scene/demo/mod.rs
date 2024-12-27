use core::mem::MaybeUninit;

use hand::HandState;
use rand::{rngs::SmallRng, RngCore as _, SeedableRng as _};
use wasm4::tracef;

use crate::{card::{deck::Deck, state::CardState}, gfx::Render, Entropy, FrameCounter, MouseSemaphore};

use super::Scene;

mod hand;

static mut DEMO: MaybeUninit<Demo> = MaybeUninit::uninit();

pub struct Demo {
    deck: Deck,
    hand: HandState,
    // hovered: Option<CardState>,
    rng: SmallRng,
    start_frame: u32,
}
impl Demo {
    pub fn init() {
        unsafe {
            DEMO = MaybeUninit::new(Self::new());
        }
    }
    fn new() -> Self {
        Self {
            deck: Deck::new(),
            hand: HandState::new(8, heapless::Vec::new()),
            rng: SmallRng::from_seed(Entropy::get()),
            start_frame: 0,
        }
    }
    pub fn get() -> &'static mut Self {
        let demo = unsafe { DEMO.assume_init_mut() };
        demo.start_frame = FrameCounter::get() + 20;
        demo
    }
}
impl Scene for Demo {
    fn update(&mut self, m: &MouseSemaphore) {
        if FrameCounter::get() < self.start_frame {
            tracef!(
                "Start frame: {}, Current: {}",
                self.start_frame,
                FrameCounter::get(),
            );
            return;
        } else if self.start_frame == FrameCounter::get() {
            // init
            // I'm throwing shit at the wall trying to get as much entropy as I can
            self.rng = SmallRng::seed_from_u64(SmallRng::from_seed(Entropy::get()).next_u64() + FrameCounter::get() as u64);
            self.deck.shuffle(&mut self.rng);

            while self.hand.cards.len() != self.hand.size {
                if let Some(card) = self.deck.draw() {
                    self.hand.push(card);
                }
            }
        }

        self.hand.cards
            .iter_mut()
            .rev()
            .for_each(|card| card.handle_input(m));

        self.hand.cards
            .iter_mut()
            .for_each(CardState::update);
    }
}
impl Render for Demo {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        if self.start_frame > FrameCounter::get() {
            fb.text(
                "Generating...",
                [50, 70],
                wasm4::draw::DrawIndex::Second,
                wasm4::draw::DrawIndex::Transparent,
            );
            return;
        }

        self.hand.render(fb);
    }
}
