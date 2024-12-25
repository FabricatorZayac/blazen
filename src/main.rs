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

use card::{card::{Card, Rank, Suit}, state::CardState};
use gfx::Render;
use wasm4::{
    self as w4, control::{Mouse, MouseState}, draw::{Color, Framebuffer}, tracef
};
use crate::animator::animation_state::AnimationState;

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

struct Blazen {
    fb: Framebuffer,
    mouse: Mouse,
    prev_mouse: Option<MouseState>,

    cards: heapless::Vec<CardState, 10>,
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
            Card::new(Suit::Heart, Rank::Two),
            [70, 70],
            None,
        )).ok();
        this.cards.push(CardState::new(
            Card::new(Suit::Spade, Rank::Two),
            [80, 80],
            None,
        )).ok();


        tracef!("Sizeof AnimationState: {}", size_of::<AnimationState>());
        tracef!("Sizeof CardState: {}", size_of::<CardState>());

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
            .for_each(|card| card.handle_input(&self.mouse));

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
