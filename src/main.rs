#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop { }
}

mod alloc;

mod button;
mod card;
mod gfx;
mod animator;
mod util;

use core::panic::PanicInfo;

use util::{Angle, Duration};
use card::{card::{Card, Rank, Suit}, state::CardState};
use gfx::Render;
use wasm4::{
    self as w4, control::{Mouse, MouseState}, draw::{Color, Framebuffer}, tracef
};
use crate::animator::animation_state::{AnimationState, Transform};

struct Blazen {
    fb: Framebuffer,
    mouse: Mouse,
    prev_mouse: Option<MouseState>,

    cards: heapless::Vec<CardState, 10>,
}

static mut LOG_BUF: [u8; 200] = [0; 200];

impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        let bufptr = &raw mut LOG_BUF;
        let buf = unsafe { bufptr.as_mut().unwrap() };
        res.logger.init(buf);

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

        this.cards[1].add_animation(AnimationState::new(
            Transform::Rotate(
                Angle::from_deg(0.0),
                Angle::from_deg(270.0),
            ),
            Duration::from_secs(3.0),
        ));

        this.cards[1].add_animation(AnimationState::new(
            Transform::Translate([100.0, 100.0]),
            Duration::from_secs(3.0),
        ));

        // this.cards[1].add_animation(AnimationState::new(
        //     Transform::Scale(2.0),
        //     Duration::from_secs(3.0),
        // ));

        this
    }

    fn update(&mut self) {
        self.mutate();
        self.render();
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

        self.cards.iter_mut()
            .map(CardState::view)
            .for_each(|view| view.render(&self.fb));

        self.prev_mouse = Some(self.mouse.state());
    }

    fn render(&self) {
        // self.cards.iter().for_each(|card| self.fb.line(
        //     card.origin(),
        //     card.origin(),
        //     wasm4::draw::DrawIndex::First)
        // );
    }
}

w4::main! { Blazen }
