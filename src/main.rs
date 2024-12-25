#![no_std]
#![no_main]

#![allow(internal_features, static_mut_refs)]
#![feature(core_intrinsics)]

use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop { }
}

// mod alloc;

// mod button;
mod card;
mod hand;
mod gfx;
mod animator;
mod util;


use card::{card::{Card, Rank, Suit}, deck::{self, Deck}, state::CardState};

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

struct MouseSemaphore {
    mouse: Mouse,
    prev_mouse: Option<MouseState>,
    lock: bool,
}
impl MouseSemaphore {
    pub fn new(mouse: Mouse) -> Self {
        MouseSemaphore { mouse, prev_mouse: None, lock: false }
    }
    pub fn state(&self) -> Option<MouseState> {
        if self.lock {
            None
        } else {
            Some(self.mouse.state())
        }
    }
    pub fn lock(&mut self) {
        self.lock = true;
    }
    pub fn update(&mut self) {
        self.unlock(); 
        self.prev_mouse = self.state();
    }

    fn unlock(&mut self) {
        self.lock = false;
    }
}

struct Blazen {
    fb: Framebuffer,
    mouse: MouseSemaphore,

    deck: Deck,
    cards: heapless::Vec<CardState, 10>,
}

impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        res.logger.init(unsafe {LOG_BUF.as_mut_slice()});

        tracef!("Hello {}!", "logger");

        let mut this = Blazen {
            fb: res.framebuffer,
            mouse: MouseSemaphore::new(res.controls.mouse),
            deck: Deck::new(),

            cards: heapless::Vec::new(),
        };

        this.cards.push(CardState::new(
            Card::new(Suit::Heart, Rank::Nine),
            [70, 70],
            None,
        )).ok();
        this.cards.push(CardState::new(
            Card::new(Suit::Spade, Rank::Seven),
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

        // tracef!("Handling input");

        self.cards
            .iter_mut()
            .rev()
            .for_each(|card| card.handle_input(&mut self.mouse));

        // tracef!("Input handled");

        self.cards
            .iter_mut()
            .for_each(CardState::update);

        // tracef!("Cards drawn");

        self.mouse.update();
    }

    fn render(&self) {
        self.cards
            .iter()
            .map(CardState::view)
            .for_each(|view| view.render(&self.fb));
    }
}

w4::main! { Blazen }
