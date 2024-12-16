#![no_main]

mod cards;

use crate::cards::deck::Deck;
use wasm4::{
    self as w4,
    draw::{Color, DrawIndex, DrawIndices, Framebuffer},
    sys,
};

enum State {
    Menu,
    Game {
        state: GameState,
        deck: Deck,
    },
}

enum GameState {
    Pause,
    Play,
    DeckView,
}

struct Blazen {
    framebuffer: Framebuffer,
    state: State,
}

// prints "tick" every second
impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        Blazen {
            framebuffer: res.framebuffer,
            state: State::Menu,
        }
    }

    fn update(&mut self) {
        self.framebuffer.replace_palette([
            Color(0x8f9bf6),
            Color(0x161616),
            Color(0xab4646),
            Color(0xf0f0f0),
        ]);

        match self.state {
            State::Menu => self.menu(),
            State::Game { state: _, deck: _ } => todo!(),
        }
    }
}

impl Blazen {
    fn menu(&mut self) {
        unsafe { *sys::DRAW_COLORS = 0x2; }
        self.framebuffer.text("BLAZEN", [55, 40]);

        unsafe { *sys::DRAW_COLORS = 0x32; }
        self.framebuffer.rect([20, 100], [120, 40]);

        unsafe { *sys::DRAW_COLORS = 0x13; }
        self.framebuffer.rect([30, 105], [43, 30]);

        unsafe { *sys::DRAW_COLORS = 0x4; }
        self.framebuffer.text("New", [40, 110]);
        self.framebuffer.text("game", [36, 120]);
    }
}

w4::main! { Blazen }
