#![no_std]
#![no_main]

#![allow(internal_features, static_mut_refs)]
#![feature(core_intrinsics)]

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop { }
}

// mod alloc;

mod button;
mod card;
mod gfx;
mod animator;
mod util;

use core::panic::PanicInfo;

use animator::transform::{Rotate, Shear, Translate};
use util::Duration;
use card::{card::{Card, Rank, Suit}, state::CardState};
use gfx::Render;
use wasm4::{
    self as w4, control::{Mouse, MouseState}, draw::{Color, Framebuffer}, tracef
};
use crate::animator::animation_state::AnimationState;

struct Blazen {
    fb: Framebuffer,
    mouse: Mouse,
    prev_mouse: Option<MouseState>,

    cards: heapless::Vec<CardState, 10>,
}

static mut LOG_BUF: [u8; 200] = [0; 200];

static mut FRAME_COUNT: u32 = 0;
struct FrameCounter;
impl FrameCounter {
    fn get() -> u32 {
        unsafe { FRAME_COUNT }
    }
    unsafe fn increment() {
        unsafe { FRAME_COUNT += 1 };
    }
}

impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        res.logger.init(unsafe {LOG_BUF.as_mut_slice()});

        tracef!("Hello {}!", "logger");

        let mut this = Blazen {
            fb: res.framebuffer,
            mouse: res.controls.mouse,
            prev_mouse: None,

            cards: heapless::Vec::new(),
        };
        this.cards.push(CardState::new(
            Card::new(Suit::Spade, Rank::Two), [80, 80])).ok();

        this.cards.push(CardState::new(
            Card::new(Suit::Heart, Rank::Two), [30, 30])).ok();

        this.cards.push(CardState::new(
            Card::new(Suit::Diamond, Rank::Two), [110, 120])).ok();

        this.cards.push(CardState::new(
            Card::new(Suit::Club, Rank::Two), [30, 120])).ok();

        this.cards.push(CardState::new(
            Card::new(Suit::Heart, Rank::Eight), [110, 30])).ok();

        // let rotation: &dyn Transform = &Rotate::new(0.0, 270.0);
        // let translation: &dyn Transform = &Translate::new([0.0, 0.0], [100.0, 100.0]);

        this.cards[1].add_animation(AnimationState::new(
            &[
                Rotate::new(0.0, 270.0).into(),
                Translate::new([0.0, 0.0], [100.0, 100.0]).into(),
            ],
            Duration::from_secs(3.0),
            Some(|| AnimationState::new(
                &[
                    Rotate::new(270.0, 0.0).into(),
                    Translate::new([100.0, 100.0], [0.0, 0.0]).into(),
                ],
                Duration::from_secs(3.0),
                None
            )),
        ));

        tracef!("Sizeof AnimationState: {}", size_of::<AnimationState>());
        tracef!("Sizeof CardState: {}", size_of::<CardState>());

        this.cards[0].add_animation(AnimationState::new(&[
            Shear::new([0.0, 0.0], [20.0, 20.0]).into()
        ], Duration::from_secs(3.0), None));

        // this.cards[1].add_animation(AnimationState::new(
        //     Transform::Scale(2.0),
        //     Duration::from_secs(3.0),
        // ));

        this
    }

    fn update(&mut self) {
        self.mutate();
        self.render();

        unsafe {FrameCounter::increment()};
    }
}

impl Blazen {
    fn mutate(&mut self) {
        self.fb.replace_palette([
            Color(0x8f9bf6),
            Color(0x161616),
            Color(0xab4646),
            Color(0xf0f0f0),
        ]);

        self.cards
            .iter_mut()
            .for_each(CardState::update);

        self.prev_mouse = Some(self.mouse.state());
    }

    fn render(&self) {
        self.cards
            .iter()
            .map(CardState::view)
            .for_each(|view| view.render(&self.fb));
    }
}

w4::main! { Blazen }
