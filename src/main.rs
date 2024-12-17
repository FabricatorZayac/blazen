#![no_std]
#![no_main]

mod cards;
mod button;
mod gfx;

use crate::cards::deck::Deck;
use button::Button;
use cards::card::Card;
use gfx::{model::{CardModel, ACE}, texture::Texture, Render};
use micromath::vector::{F32x2, I32x2};
use wasm4::{
    self as w4, control::{Mouse, MouseState}, draw::{Color, DrawIndex, Framebuffer}, sys::{blit, BLIT_1BPP}
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
        w4::trace("Starting game");
        *self = State::Game {
            state: GameState::Inspect,
            deck: Deck::new(),
        };
    }
}

enum GameState {
    Inspect,
    Play,
}

struct Blazen {
    framebuffer: Framebuffer,
    state: State,
    mouse: Mouse,

    prev_mouse: Option<MouseState>,
}

// prints "tick" every second
impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        Blazen {
            framebuffer: res.framebuffer,
            state: State::Menu,
            mouse: res.controls.mouse,

            prev_mouse: None,
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
                GameState::Inspect => { },
                GameState::Play => { },
            },
        }

        self.prev_mouse = Some(self.mouse.state());
    }
}

impl Blazen {
    fn menu(&mut self) {
        // self.framebuffer.text("BLAZEN", [55, 40], DrawIndex::Second, DrawIndex::Transparent);
        self.framebuffer.rect([20, 100], [120, 40], DrawIndex::Second, DrawIndex::Third);

        Button::new(
            [45, 105],
            "New game",
            DrawIndex::Third,
            DrawIndex::First,
            DrawIndex::Fourth,
            |s|{ s.state.start_game(); },
        ).update(self).render(&self.framebuffer);

        CardModel {
            origin: I32x2 { x: 65, y: 50 },
            card: &Card::new(cards::card::Suit::Spade, cards::card::Rank::Ace),
            texture: [
                Texture {
                    buf: &ACE,
                    uv: [
                        F32x2 { x: 0.0, y: 0.0 },
                        F32x2 { x: 1.0, y: 0.0 },
                        F32x2 { x: 0.0, y: 1.0 },
                    ]
                },
                Texture {
                    buf: &ACE,
                    uv: [
                        F32x2 { x: 1.0, y: 0.0 },
                        F32x2 { x: 1.0, y: 1.0 },
                        F32x2 { x: 0.0, y: 1.0 },
                    ]
                }
            ]
        }.render(&self.framebuffer);
    }
}

w4::main! { Blazen }
