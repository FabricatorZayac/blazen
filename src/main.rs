#![no_main]

mod cards;
mod button;

use crate::cards::deck::Deck;
use button::Button;
use wasm4::{
    self as w4,
    draw::{Color, DrawIndex, Framebuffer},
};

enum State {
    Menu,
    Game {
        state: GameState,
        deck: Deck,
    },
}

impl State {
    fn start_game(&mut self) {
        *self = State::Game {
            state: GameState::Inspect,
            deck: Deck::new(),
        };
    }
}

enum GameState {
    Pause,
    Play,
    Inspect,
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

        match &self.state {
            State::Menu => self.menu(),
            State::Game { state, deck } => match state {
                GameState::Pause => todo!(),
                GameState::Play => todo!(),
                GameState::Inspect => todo!(),
            },
        }
    }
}

impl Blazen {
    fn menu(&mut self) {
        self.framebuffer.text("BLAZEN", [55, 40], DrawIndex::Second, DrawIndex::Transparent);
        self.framebuffer.rect([20, 100], [120, 40], DrawIndex::Second, DrawIndex::Third);

        Button::new(
            [45, 105],
            "New game",
            DrawIndex::Third,
            DrawIndex::First,
            DrawIndex::Fourth,
            ||{ self.state.start_game(); },
        ).draw(&self.framebuffer);
    }

    fn pause(&mut self) {

    }
}

w4::main! { Blazen }
