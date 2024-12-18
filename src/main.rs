#![no_std]
#![no_main]

mod button;
mod card;
mod gfx;

use card::{card::Card, state::CardState, view::CardView};
use gfx::Render;
use wasm4::{
    self as w4,
    control::{Mouse, MouseState},
    draw::{Color, Framebuffer},
};

struct Blazen {
    framebuffer: Framebuffer,
    mouse: Mouse,
    prev_mouse: Option<MouseState>,

    card: CardState,
}

// prints "tick" every second
impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        Blazen {
            framebuffer: res.framebuffer,
            mouse: res.controls.mouse,
            prev_mouse: None,

            card: CardState::new(Card::new(
                card::card::Suit::Spade,
                card::card::Rank::Two,
            ), (80.0, 80.0).into()),
        }
    }

    fn update(&mut self) {
        self.mutate();
        self.render();
    }
}

impl Blazen {
    fn mutate(&mut self) {
        self.framebuffer.replace_palette([
            Color(0x8f9bf6),
            Color(0x161616),
            Color(0xab4646),
            Color(0xf0f0f0),
        ]);

        self.card.rotate(0.01.into());

        self.prev_mouse = Some(self.mouse.state());
    }

    fn render(&self) {
        self.card.view().render(&self.framebuffer);
    }
}

w4::main! { Blazen }
