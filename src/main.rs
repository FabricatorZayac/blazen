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

use core::panic::PanicInfo;

use card::{card::Card, state::CardState};
use gfx::Render;
use wasm4::{
    self as w4, control::{Mouse, MouseState}, draw::{Color, Framebuffer}, tracef
};
use crate::animator::animation_manager::AnimationManager;
use crate::animator::animator::{Animator, Transformation};

struct Blazen {
    fb: Framebuffer,
    mouse: Mouse,
    prev_mouse: Option<MouseState>,

    cards: heapless::Vec<CardState, 10>,
    card_animators: AnimationManager,
}

static mut LOG_BUF: [u8; 200] = [0; 200];

// prints "tick" every second
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
            card_animators: AnimationManager::new(),
        };
        this.cards.push(CardState::new(Card::new(
            card::card::Suit::Spade,
            card::card::Rank::Two,
        ), ( 80.0, 80.0 ).into())).unwrap();

        this.cards.push(CardState::new(Card::new(
            card::card::Suit::Heart,
            card::card::Rank::Two,
        ), ( 30.0, 30.0 ).into())).unwrap();

        this.cards.push(CardState::new(Card::new(
            card::card::Suit::Diamond,
            card::card::Rank::Two,
        ), ( 110.0, 110.0 ).into())).unwrap();

        this.cards.push(CardState::new(Card::new(
            card::card::Suit::Club,
            card::card::Rank::Two,
        ), ( 30.0, 110.0 ).into())).unwrap();

        this.cards.push(CardState::new(Card::new(
            card::card::Suit::Heart,
            card::card::Rank::Eight,
        ), ( 110.0, 30.0 ).into())).unwrap();

        let card_rotation_animator = Animator::new(
            Transformation::Rotation { start: 0.0, end: 270.0 },
            3000, // 3 seconds
        );
        this.card_animators.add(card_rotation_animator);

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

        // for card in &mut self.cards {
        //     card.rotate(0.01.into());
        // }

        self.card_animators.update(&mut self.cards[1], (1.0/60.0*1000.0) as u32);

        self.prev_mouse = Some(self.mouse.state());
    }

    fn render(&self) {
        for card in &self.cards {
            card.view().render(&self.fb);
        }
    }
}

w4::main! { Blazen }
